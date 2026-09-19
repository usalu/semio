# AU3 — Live sign-in: closing the loop between AU1 (hub auth) and AU2 (os UI)

Slice AU3 of ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`, started 2026-09-19 (Opus).
Inputs read in full before touching anything: `📓️worker-preamble.md`, `📓️status.md`, AGENTS.md,
`📓️au1-hub-auth-sessions-and-rate-limit.md` (esp. §1 contract and §7 gaps),
`📓️au2-os-sign-in-and-spaces-ui.md` (esp. §6 gaps), `📓️h1-hub-build-and-boot.md`,
`📓️u1-progress-cancel-and-connection-status.md`, `📓️z1-zero-touch-and-launch-rows.md`.

Status legend: ✅ landed and executed · 🚧 landed, not executed · ❌ not done.

---

## 1. Measured truth — the AU1/AU2 discrepancy resolved ✅

**AU1 is right; AU2's measurement was a false negative.**

AU2 §1 reported `grep -c 'route("/auth/sessions"' 🌎️hub/🏗️bootstrap/🦀️.rs` → `0` and concluded the
route was unmounted. It is mounted — but through a **constant**, not a literal, so the literal grep
could not see it:

```
🌎️hub/🏗️bootstrap/🦀️.rs:8299
  .route(semio_hub::auth::SESSION_MINT_ROUTE, post(post_auth_session)
      .layer(DefaultBodyLimit::max(semio_hub::auth::SIGN_IN_REQUEST_MAX_BYTES)))
🌎️hub/🏗️bootstrap/🦀️.rs:8345
  .layer(axum::middleware::from_fn_with_state(state.clone(), rate_limit_middleware))
```

with `🌎️hub/🔐️auth/🦀️.rs:29  pub const SESSION_MINT_ROUTE: &str = "/auth/sessions";`.

So AU2's §5 "there is nothing to sign in to yet" and §6 gap 1 are **retracted**: the hub route
existed when AU2 finished. What was genuinely missing is everything in §2–§4 below.

_(sections filled as the slice proceeds)_

## 2. Dev relay admission — `/auth/sessions` ✅

`🌎️hub/🚀️local-relay/🧭️routing/🟦️.ts` admitted **only** `GET /auth/sessions/me`. A browser reaching
the hub through the dev relay could therefore read a session it had no way to mint, end or re-key.
Now admitted, verb by verb (`🟦️.ts:70-74`):

| method | path | why |
|---|---|---|
| `POST` | `/auth/sessions` | credential sign-in |
| `GET` | `/auth/sessions/me` | session introspection (pre-existing) |
| `DELETE` | `/auth/sessions/me` | sign-out |
| `POST` | `/auth/credentials` | password change (§3.2) |
| `POST` | `/directory/invites/{token}/redeem` | invite redemption — the last route AU2's UI needs and the only one left outside the allowlist; bounded by a new `localRelayInviteRedemptionPath` (`🟦️.ts:52-64`) to the hub's own selector charset, so a traversal, an encoded separator, an empty or a >512-byte token never reaches the upstream |

**Honest limit, stated because it changes what the relay lane means.** The relay *replaces* the
caller's `Authorization` header with its own local-bootstrap capability
(`🌎️hub/📦️packages/🦀️rust/📜️script.ts:534`). So a sign-in performed *through* the relay mints a real
session and hands the token back to the browser, but every later relay call still acts as the
bootstrap principal, not as that human. The relay is the zero-touch dev lane for a machine-local
operator; per-user sign-in is a *different* channel, and §4.3 points the os workspace at the hub's
own origin for exactly this reason. Admitting these routes is still right — the relay must be able
to end and re-key the session it holds — but it is not a per-user auth path, and this slice does not
pretend it is.

**Tests.** New file `🌎️hub/🚀️local-relay/🧭️routing/🧪️tests/🔬️admission/🟦️.ts`: **4 laws, 85
assertions, 0 failures** (`🗑️generated/au3-relay-admission.txt`). Every admitted row is asserted
against the route constant the hub's own `🔐️auth` scope exports, so a renamed route breaks the test
rather than the product; the wrong verb, any query string, ten look-alike paths and ten hostile
invite selectors are all refused; and the five pre-existing admitted families are re-asserted
unchanged. Registered as nx target **`os-hub:local-relay-routing-check`**
(`📜️script.ts:99,16649` + `📋️project.json`), run green through its own script entry.

Why a new file rather than extending `🌎️hub/🧪️tests/🧱️socket-grant-command-source/🟦️.ts` (which is
where the relay is exercised today): that file's nx target `socket-grant-command-source-check` **does
not exist at HEAD** — a peer replaced it with `foundation-source-check` — so the file is orphaned and
its last two laws assert against the removed target and a renamed `namedInputs` key. I put the laws
somewhere that actually runs, and fixed only the one drift in its fixture that I could attribute
(`🧫️fixtures/🧱️socket-grant-command-source/🔣️.json` `rootImports` order, now matching the real
import). The remaining two failures in that orphaned file are a peer's in-flight refactor; see §7.

## 3. Hub

### 3.1 `GET /auth/sessions/me` already carries `expiresAtMs` — nothing to add ✅

Measured rather than assumed: the route is mounted (`🏗️bootstrap/🦀️.rs:8377`), its handler
`get_session_me` (`:6532`) already serves `DirectorySessionAuthorityV1`, and that record already
carries `expires_at` (serialized `expiresAt`, epoch ms) beside `userId`, `email`, `displayName`,
`sessionKind` and `authorizationGeneration`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🦀️.rs:18-30`).
The introspection gap was therefore **entirely on the os side** — nobody called it. That is §4.2.
The route constant it was mounted under is now `semio_hub::auth::SESSION_ME_ROUTE` so the three auth
routes are named in one place.

### 3.2 `POST /auth/credentials` — the password-change command ✅

Schema first: `$defs/CredentialChangeRequestV1` in `🌎️hub/🔐️auth/🧬️schema/🔣️.json`
(closed, camelCase, both secrets 8..=256), TypeScript twin `🟦️.ts` (`AUTH_CREDENTIAL_ROUTE`,
`CREDENTIAL_CHANGE_SCHEMA`, `credentialChangeRequestV1()`), Rust decoder
`CredentialChangeRequestV1` + `VerifiedCredentialChangeV1::verify` (`🔐️auth/🦀️.rs`), pure law
`decide_credential_change` (same file, `🔖️Decision` region), handler `post_auth_credential`
(`🏗️bootstrap/🦀️.rs:6644`), mounted at `:8376` inside the same 1024-byte body limit and the same
`RateLimitClassV1::Auth` bucket as sign-in (`:6783`).

Design decisions, defended:
- **Two independent proofs.** A live bearer says *which* principal; the current password says the
  human is present. A stolen capability alone can never take an account over.
- **Success revokes every session of that principal, including the caller's** — a password change is
  exactly the moment you want any session the old password leaked to stop working. Socket grants and
  open plans bound to each revoked session are invalidated in the same pass.
- **A principal with no credential is refused exactly like a wrong password** (`invalid-credentials`)
  — the first credential is issued by §3.3's operator verb, which is not reachable over the network.
- **An unchanged password is a `400`**, not a silent re-salt that would journal a `credential-changed`
  fact that changed nothing.
- **Only refusals are journaled here**: a success journals its own fact *inside*
  `set_password_credential`'s transaction, so the column and the log can never disagree.

### 3.3 First-user / bootstrap story — an operator verb on the binary ✅

**The question.** A fresh `OS_HUB_DATA` has no principal with a password. The local-bootstrap pipe
creates one (`🚀️local-bootstrap/🦀️.rs:688`) but with a synthetic `local-<digest>@bootstrap.invalid`
email and **no** credential, and it only mints `development-local` sessions.

**The choice, and why.** I implemented the **operator CLI verb**, not a local-bootstrap credential
claim, because it is the one with maximum control:

```
OS_HUB_DATA=/abs/path os-hub credential set --email a@example.com [--display-name "Ada"] < password
```

`🌎️hub/🔐️auth/📤️command/🦀️.rs` (new), dispatched in `main` before any service is opened
(`🏗️bootstrap/🦀️.rs:8556`), mirroring the existing `trusted-catalog publish` verb exactly.

- It requires **read/write access to the hub's server-owned data root** — strictly stronger than any
  capability reachable over the network. No bearer, no admin subject and no "first request wins"
  race can stand in for physical control of the deployment. A local-bootstrap claim route would have
  been reachable by anything holding the pipe, which in dev is the *launcher*, i.e. any process the
  dev server spawns.
- The password is read from **stdin** — never `argv` (world-readable via `ps`), never an environment
  variable (inherited by children) — bounded to 8..=256 bytes, and the buffer is zeroed after mint.
- It works whether or not the hub is running, and it is idempotent in the useful sense: an absent
  account is created with the credential; an existing account keeps its identity, gets the new
  credential plus a `credential-changed` fact, and has every live session revoked.
- `--email` is lowercased and bounds-checked with exactly the route's own admission, so an operator
  can never provision an account the sign-in route would refuse to look up.

5 unit laws in the module (verb selection, email bounds + lowercasing, display-name bounds, the
stdin password boundary including the UTF-8 and ceiling cases, the iteration bound).

### 3.4 Postgres and Neo4j credential paths — implemented, no longer failing closed ✅

AU1 §7 gap 2 left both backends on the trait's erroring defaults, so a hub on either backend refused
**every** sign-in with `503`. Both now mirror the sqlite implementation:

| backend | file | what |
|---|---|---|
| Postgres | `🌎️hub/📇️directory/🐘️postgres/🦀️.rs` (after `list_auth_audit`) | `append_credential_audit` (one transaction) and `set_password_credential` (`UPDATE hub_user … RETURNING`-checked row count + the fact, one transaction; an absent user is a `Conflict`, never a silent no-op) |
| Neo4j | `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs` (after `list_auth_audit`) | the same pair over `MATCH (u:User {id}) SET u.passwordHash … RETURN u.id`, rolling the transaction back when nothing matched |

Both compile clean under their own feature (`🗑️generated/au3-hub-{postgres,neo4j}-check.txt`), and
their **test** builds are green too — which required fixing a pre-existing `#[derive(Default)]` over
`tokio::sync::Semaphore` in each backend's `#[cfg(test)] ArtifactGenesisTestControlV1` (2 × E0277 per
backend; the two rendezvous gates now start closed explicitly, which is what the tests want).

## 4. os

### 4.1 `HubWorkspace` is reachable from the shell ✅ (AU2 §6 gap 2)

- **Command-palette / action-line verb** `os.openHub` — `OPEN_HUB_COMMAND_ID`
  (`🛠️ShellHelpers/🟦️.tsx:2570`), definition in `buildOsCommands` (`:4645`, category `general`, icon
  `users`, `inPalette: true`), handled in `ShellHost`'s own `onCommand` beside the other os commands
  that need shell state. New label key `ui.command.openHub`, **en + de** (the bundle's type makes
  both a compile-time requirement).
- **The persistent hub-connection indicator U1 added now opens it.** `hubSessionPresence` was a
  documented constant `"none"` (`🏛️ShellHost/🟦️.tsx`), which is exactly "offer no button". It is now
  shell state seeded `"signedOut"`, and the indicator gets `onSignIn={openHubWorkspace}` — U1's
  `data-semio-hub-sign-in` button becomes live. `HubWorkspace` reports its session phase back through
  a new `onSessionChange` prop, so closing the overlay does not make a live session look absent (the
  workspace unmounts; the port, and therefore the capability, does not).
- Both paths go through `navigateHistory("/hub")`, so the verb, the badge and a pasted `/hub` link
  are one code path.

### 4.2 Expired-session re-auth through `me` ✅ (AU2 §6 gap 4)

- New contract functions in `📇️directory/🔐️sign-in/🟦️.ts`: `parseHubSessionAuthorityV1` (decodes the
  hub's own closed `DirectorySessionAuthorityV1`, refusing an unknown schema, a non-integer or
  zero `expiresAt`, an unknown `sessionKind`, an over-long display name and a proxy's HTML) and
  `runHubSessionAuthorityV1` (`authority` | `expired` | `failed`; a `401` is `expired`).
- `useHubConnection` calls it **right after every mint**, so `session.expiresAtMs` is the hub's own
  deadline instead of `null`, and re-arms a timer against that deadline (capped at 30 min per sleep,
  because a one-year TTL overflows `setTimeout`). A `401` from that read drives the session to
  `expired`, which is what `hubSessionNeedsReauthenticationV1` already keys on.

### 4.3 `members` fed from the real directory projection ✅ (AU2 §6 gap 3)

`ShellHost` passed `members={[]}`. Now:
- the port gained `readSpaceMembers(origin, spaceId, signal)` → `GET /directory/spaces/{id}`, whose
  `members` window (`DirectorySpaceAdministrationMemberRowV1`) is the hub's only authoritative
  roster; a non-member's `404` is an **empty roster**, not an error;
- `parseHubSpaceMemberRowsV1` bounds-checks every row rather than trusting the wire;
- `useHubConnection(port, onlineUserIds)` joins it with presence through the existing
  `spaceMemberPresenceV1`, and `HubWorkspace` renders `hub.members`;
- `ShellHost` supplies `hubOnlineUserIds` — the **hub identities** its presence lane sees
  (`PresencePeer.userId`), deduped; a peer with no hub identity contributes nothing, so the roster
  can never show a member the directory does not know.

### 4.4 The hub is its own origin ✅ — the integration defect neither AU1 nor AU2 could see

`ShellHost` built the port with `bootstrapOrigin: globalThis.location.origin`, i.e. the **UI**
origin. In the dev topology the vite server proxies only `/_semio` to the relay
(`🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts:154-161`); `POST <uiOrigin>/auth/sessions` hits the SPA's own
index route and returns HTML. A sign-in could not have worked from the shell as AU2 left it, on any
topology where hub and UI are not literally co-served.

Fixed with `hubBootstrapOriginV1()` (`🏛️ShellHost/🟦️.tsx`): it reads the `VITE_S_HUB_URL` define the
build **already injects** as non-secret collaborative endpoint metadata, normalizes it to a bare
origin (`parseHubOriginV1` refuses a path/query/fragment), and falls back to the page origin only
when no hub is declared. The hub reflects the request's own `Origin` with
`access-control-allow-credentials` and allows `authorization, content-type`
(`🏗️bootstrap/🦀️.rs:6733`), so the browser reaches it directly with the human's own bearer — which
is the whole point: the session capability belongs to the browser, not to the relay.

## 4.5 The contract conflict AU1's feature created — fixed at its source ✅

A development hub **could not be booted with credential sign-in enabled at all**, which is why
nobody had ever signed in live. `🚀️local-bootstrap/🧬️schema/🔣️.json:193` pinned
`authentication.publicSessionIssuance` to `{ "const": false }`, and every local run's readiness
classifier (`🚀️local-bootstrap/🏃️execution/🟦️.ts:208`) threw `hub readiness binding mismatch` on
anything else. AU1's `declare_public_session_issuance` correctly reports the *policy*, so
`OS_HUB_CREDENTIAL_SIGN_IN=1` made `/readyz` say `true` and the launcher refused its own hub.

Resolved where the meaning lives rather than by weakening either side: the field is a **boolean**
with its meaning written down, and the classifier compares it against **the run's own expectation**
instead of a constant. `LocalHubRun` now carries `publicSessionIssuance`, derived from the exact env
`startLocalHub` hands the child (`🏃️execution/🟦️.ts:47,145,175,214,231`). A run that did not ask for
public issuance still fails closed if the hub offers it — the property the `const` was protecting is
kept, and a run that *did* ask is admitted.

## 5. Live transcript — the point of this slice ✅

`🌎️hub/🔐️auth/🧪️tests/🤝️live-sign-in/🟦️.ts` (promoted from the ticket's
`🐍️au3-live-sign-in-probe.ts`) boots the real `os-hub` binary against a **brand new** `OS_HUB_DATA`,
provisions the first two principals through the operator verb, and drives the whole lifecycle over
real HTTP. Capture: `🗑️generated/au3-live-transcript.txt` (and `au3-live-gate.txt` through the nx
script entry). **38 checks, 0 failures.**

```
fresh OS_HUB_DATA=/private/tmp/au3-hub-data-…  port=8831  credentialSignIn=1
operator bootstrap: ada=01a0b8ed-55d9-… bo=01a0b8ed-64f2-…
PASS 1  sign-in status 200 · token matches session.v1.<32hex>.<64hex> · user id is the provisioned one
PASS 2  me 200 · userId/email/displayName/sessionKind=external · expiresAt is a future epoch-ms integer
PASS 3  wrong password 401 invalid-credentials · an unknown account is byte-identical (no enumeration)
PASS 4  create-space 202 accepted · the new space appears in the author's own /directory/spaces
PASS 5  create-invite 202 · result.kind=invite · invite.v1.<32hex>.<64hex>
PASS 6  second principal signs in 200 · redeems the invitation 200 · now sees the space
PASS 7  the author's space page carries the member window · roster = [[ada,author,true],[bo,spectator,false]]
PASS 8  password change 204 · the caller's own capability is now 401 · the NEW password mints 200 ·
        the OLD password no longer mints (401) · a change without the current password is 401
PASS 9  sign-out 204 · the signed-out capability is 401
PASS 10 the sign-in bucket refuses with 429 + retry-after (integer seconds ≥ 1) ·
        a CORRECT password is refused while the bucket is empty
au3-live-sign-in: all checks passed
```

Every command is posted through the **production** `sealDirectoryCommandRequestV1` /
`directoryCommandRequestJson`, so the gate posts byte-for-byte what the shell posts rather than a
hand-rolled envelope.

### 5.1 Two defects only the live hub could find

1. **`members` is a window, not an array.** `GET /directory/spaces/{id}` serves
   `DirectorySpaceAdministrationPageV1`, whose `members` is a
   `DirectorySpaceAdministrationWindowV1` — `{ rows, nextCursor? }` — and whose `access: "public"`
   projection carries no `members` key at all. My first `parseHubSpaceMemberRowsV1` read `.members`
   as a bare array, and **its unit test passed**, because I had written the fixture to match my own
   wrong assumption. The live roster returned `undefined.map`. Fixed at
   `🔗️HubConnection/🟦️.tsx` (`parseHubSpaceMemberRowsV1` now reads `members.rows`) and the law was
   rewritten against the shape the running hub actually served, including a vector asserting that a
   bare array is **refused**. This is the concrete answer to "why a live proof": a mocked roster
   agreed with itself for as long as nobody asked the hub.
2. **The readiness contract conflict** in §4.5 — invisible to both AU1 (who never booted a hub with
   the flag on) and AU2 (who never booted one at all).

### 5.2 Registered as a permanent gate ✅

- `🌎️hub/🔐️auth/🧪️tests/🤝️live-sign-in/🟦️.ts` + `…/🏃️execution/🟦️.ts` (`HubLiveSignInScript`).
- `📜️script.ts` verb **`live-sign-in-check`**; `📋️project.json` target `os-hub:live-sign-in-check`
  (`cache: false`, `dependsOn: ["build-dev"]` — a live gate must never serve a cached verdict).
- The gate resolves the Nx-staged binary first and falls back to the shared cargo cache's debug
  build, so it runs both through its target and from a bare `bun` during development.
- Launch rows `⚖️gate🔐️hub-auth🤝️live-sign-in` and `⚖️gate🧭️local-relay📛️admission` inserted
  identically into `.vscode/🧩️launch.seed.jsonc` **and** `.vscode/launch.json` (group `4_gate`,
  orders `411.10757` / `411.10756`, beside the existing hub gates).

Run green through its own script entry (`bun ./📜️script.ts live-sign-in-check`,
`🗑️generated/au3-live-gate.txt`). Per preamble rule 16 I did not put the nx wrapper in front of it.

### 5.3 Two humans, two browsers, one live hub ✅

`🐍️au3-browser-probe.mjs` drives **two isolated Chromium contexts** at 1440×900 against the React
shell (`http://127.0.0.1:6081`, `S_HUB_URL=http://127.0.0.1:8831`) and the same live hub.
**19 checks, 0 failures** (`🗑️generated/au3-browser.txt`), with console, network and screenshots
captured (`au3-browser-{console,network}.txt`, `au3-browser-{ada,bo}.png`).

```
PASS 1 the shell renders the persistent hub-connection badge · it offers a sign-in entry point
PASS 2 the badge opens the hub workspace · the workspace names the hub it is bound to
PASS 3 a wrong password shows exactly one denial region
PASS 4 the real credential signs in · the badge stops offering sign-in ·
       the browser reached POST /auth/sessions · the shell read GET /auth/sessions/me for its expiry
PASS 5 create-space goes through /directory/commands · the row is rendered from the hub's own listing
       and carries the hub's own space id
PASS 6 the invitation is a one-shot invite.v1.<32hex>.<64hex> capability link
PASS 7 a SECOND human signs in in an isolated context · pastes the link · the hub admits the
       redemption · the second human now sees the shared space
PASS 8 both sign out · both badges offer sign-in again · the browser reached DELETE /auth/sessions/me
au3-browser: all checks passed
```

The captured hub traffic, which is the part no mock can fake:

```
[ada] 200 GET /directory/spaces
[ada] 401 POST /auth/sessions          ← wrong password
[ada] 200 POST /auth/sessions          ← the real credential
[ada] 200 GET  /auth/sessions/me       ← §4.2's follow-up, in a real browser
[ada] 200 GET /directory/spaces
[ada] 202 POST /directory/commands     ← create-space
[bo]  200 POST /auth/sessions          ← a second human, isolated context
[bo]  200 GET  /auth/sessions/me
[bo]  200 POST /directory/invites/invite.v1.5fa3d9f4….f97195cb…/redeem
[bo]  200 GET /directory/spaces        ← the shared space is now his
[ada] 204 DELETE /auth/sessions/me
```

The screenshot `au3-browser-ada.png` shows the workspace rendering the hub's own listing with the
shared space reading **"Author · 2 members"**.

### 5.4 A third live-only defect: the entry point was dead outside the hub host

The badge rendered and its button was clickable, but nothing happened. `applyShellUri` — where AU2's
`/hub` route branch lives — is guarded by `if (!hostMode || loadedPlugins.length === 0) return`
(`🏛️ShellHost/🟦️.tsx:6163-6167`), so a `/hub` navigation never reaches the branch in any plugin
playground. AU2's surface was reachable **only inside the `s` hub host**, and the entry point I added
would have been a dead button everywhere else.

Fixed by making the overlay what it is — shell state — and the URI what it is — host-mode addressing
(`🏛️ShellHost/🟦️.tsx`, `openHubWorkspace` and the overlay's `onClose`): the workspace opens directly,
and only a shell that owns a router also records `/hub`. Signing in to a hub is shell chrome, not a
host-mode privilege.

### 5.5 Why the `s` hub playground was not the browser host

`bun ./📜️script.ts activate s react dev` fails on the first missing staged plugin module
(`🔌️plugin/…/dist/dev/🔌️plugin-modules/🧱️block`, capture `🗑️generated/au3-activate-s-react.txt`):
the `s` variant's activation needs `prepare-s-react-dev`, i.e. materializing ~20 plugin wasm
components, of which `🧱️block` is the one absent from the 58 already staged. That is slice B3a's
in-flight work and an hours-long build.

The surface under test here is **`ShellHost` + `HubWorkspace`**, which every React playground mounts
identically, so the already-activated `animate` playground proves exactly the same wiring — and §5.4
is the proof that it proves *more*: the host-only gate is a defect the `s` host would have hidden.
Recorded as a gap, not hand-waved: nothing here has been observed inside the `s` hub host.

## 6. Tests and real counts

Every number below is from a run I executed; captures named.

| suite | command | result | capture |
|---|---|---|---|
| **live hub, two principals, real HTTP** | `bun ./📜️script.ts live-sign-in-check` (`os-hub:live-sign-in-check`) | **38 checks / 0 failed** | `au3-live-gate.txt`, `au3-live-transcript.txt` |
| local relay admission | `bun ./📜️script.ts local-relay-routing-check` (`os-hub:local-relay-routing-check`) | **4 laws / 85 assertions / 0 failed** | `au3-relay-admission.txt` |
| os hub surface + AU1 drift oracle | `bun ./📜️script.ts hub-sign-in-spaces-check` | **60 passed / 0 failed**, `hub-auth-contract-oracle: checks=15 clean` | `au3-au2-drift-gate.txt` |
| sqlite credential lifecycle | `cargo test -p semio-hub --lib directory::sqlite::tests::credential_writes…` | **1 passed / 0 failed** | inline |
| hub bin compile | `cargo check -p semio-hub --bin os-hub` | green, 7 warnings (proof the expansion ran) | `au3-hub-bin-check.txt` |
| postgres feature | `cargo check -p semio-hub --no-default-features --features postgres` | green | `au3-hub-postgres-check.txt` |
| neo4j feature | `cargo check -p semio-hub --no-default-features --features neo4j` | green | `au3-hub-neo4j-check.txt` |
| postgres test build | `cargo test … --features postgres --lib --no-run` | green (was 2 × E0277 before §3.4's `Default` fix) | `au3-postgres-test-build.txt` |
| neo4j test build | `cargo test … --features neo4j --lib --no-run` | green (same) | `au3-neo4j-test-build.txt` |
| os hub-surface typecheck | `bunx tsc --noEmit -p 🔣️au2-tsconfig.json` | **0 diagnostics in any file this slice owns**; 102 pre-existing elsewhere (5 in `ShellHost`, none at my lines) | `au3-typecheck-2.txt` |

The AU2 suite went **55 → 60**: 5 new laws from this slice (authority read after mint; a `401`
authority read is `expired`, not signed-in; the closed authority decoder against 8 hostile records
plus a proxy page; the roster read + presence join; the member-window decoder against 4 shapes and 5
hostile rows), and 2 of AU2's own laws were updated because the behaviour genuinely changed — the
shell now reads `me` after every mint, so a fake hub that answers `401` there is a hub that revoked
the session it just issued.

## 7. Honest gaps

1. **Nothing here was observed inside the `s` hub host.** §5.5: the `s` activation needs ~20 plugin
   wasm modules materialized and dies on the missing `🧱️block` (slice B3a's work). The browser proof
   ran in the `animate` playground, which mounts the same `ShellHost`/`HubWorkspace`. The one thing
   this cannot cover is any behaviour that is *only* reachable in host mode — specifically the `/hub`
   deep link and the space-scoped panes in gap 2.
2. **The invite-composition pane and the member roster need a space the SHELL has open.**
   `SpaceBrowser`'s invite form renders only for the `activeSpaceId` row, and `HubWorkspace` watches
   that same id, which outside host mode is always `null`. So in the browser proof the invitation was
   minted over HTTP with Ada's own credential and only the **redemption** pane was driven; the
   roster aside reads "No one else is here". The roster join itself is proven live end-to-end by the
   §5 gate (both principals, correct roles and ownership) and by the unit laws. The product fix is a
   space **selection** inside the workspace — one callback on `SpaceBrowser`, whose 24 laws would
   need updating; I did not take a peer's element's contract for it.
3. **Presence `online` is never `true` in any run I made.** `hubOnlineUserIds` reads
   `PresencePeer.userId` from the shell's presence lane, which is per attached *document*; with no
   document open in a space, the lane is empty. The join is proven by unit law (a peer with a hub
   identity is `online`, one without contributes nothing), never at runtime.
4. **The relay lane still cannot carry a per-user session** (§2): it replaces the caller's
   `Authorization` with its own bootstrap capability. The os workspace therefore talks to the hub
   origin directly (§4.4). Making the relay forward a caller-supplied bearer when one is present is
   a small, well-scoped change and would give the browser a same-origin path; I did not make it
   because it changes the relay's security posture and belongs with its owner.
5. **Postgres and Neo4j credential paths are compiled, not executed.** Both features and both test
   builds are green and the laws are written, but `docker info` fails on this machine, so the two
   backend suites did not run. The sqlite twin of the same law runs and passes.
6. **`POST /auth/credentials` has no os surface.** The route, its schema, its TS twin and its live
   behaviour (§5, four checks) exist; no pane calls it. A change-password form in `🔐️HubSignIn` is
   the remaining work.
7. **`hubSessionPresence` mirrors the workspace, so a hub session minted before this shell mounted
   reads `signedOut` until the workspace is opened once.** The capability lives in the port closure
   and still works; only the badge's wording is stale. Lifting `useHubConnection` to the shell would
   fix it at the cost of running the spaces lane always.
8. **`os.openHub` has no keybinding.** It is in the palette and on the badge; no chord was assigned
   because the shell's chord space is not mine to allocate.
9. **The orphaned `🌎️hub/🧪️tests/🧱️socket-grant-command-source/🟦️.ts` still fails 2 of 9 laws** —
   its nx target was deleted at HEAD and its fixture still names `hubSocketGrantCommandSources` and
   `socket-grant-command-source-check`, both replaced by `hubFoundationSources` /
   `foundation-source-check`. I fixed only the one drift I could attribute (`rootImports` order) and
   left the peer's rename to its owner. My relay laws live in a file that actually runs.
10. **I rewrote `🌎️hub/🔐️auth/🧬️schema/🔣️.json`'s whitespace.** Adding the `$def` through a JSON
    round-trip reflowed the file; I restored the repo's one-line-object style, so the diff against
    HEAD is now AU1's additions plus mine, but a few lines will read as reformatted. Content is
    byte-identical in meaning and AU2's `hubAuthContractOracle` (15 checks) stays clean.
11. **The browser probe is a ticket-folder probe, not an nx gate.** It needs an activated playground
    and a served UI, which a gate cannot self-provision; the self-provisioning gate is
    `os-hub:live-sign-in-check` (§5.2). The exact command to re-run the browser proof is in §9.
12. **Rate-limit ordering is load-bearing in both probes.** The sign-in bucket is keyed per address,
    and the two browsers plus the probe share `127.0.0.1`, so the lockout step runs last and the
    out-of-band invitation mint honours `retry-after`. Two earlier runs failed exactly here; that is
    the limiter working, not a flake.

## 8. Files changed

### New

| file | what |
|---|---|
| `🌎️hub/🔐️auth/📤️command/🦀️.rs` | the operator verb `os-hub credential set` + 5 unit laws |
| `🌎️hub/🔐️auth/🧪️tests/🤝️live-sign-in/🟦️.ts` | the live end-to-end gate (38 checks) |
| `🌎️hub/🔐️auth/🧪️tests/🤝️live-sign-in/🏃️execution/🟦️.ts` | `HubLiveSignInScript` |
| `🌎️hub/🚀️local-relay/🧭️routing/🧪️tests/🔬️admission/🟦️.ts` | the relay admission laws (4 laws / 85 assertions) |
| `🌎️hub/🚀️local-relay/🧭️routing/🧪️tests/🏃️execution/🟦️.ts` | `LocalRelayRoutingScript` |

### Modified

| file | change |
|---|---|
| `🌎️hub/🚀️local-relay/🧭️routing/🟦️.ts` | `+localRelayInviteRedemptionPath`; admits `POST /auth/sessions`, `DELETE /auth/sessions/me`, `POST /auth/credentials`, bounded invite redemption |
| `🌎️hub/🔐️auth/🦀️.rs` | `SESSION_ME_ROUTE`/`CREDENTIAL_ROUTE`/`CREDENTIAL_CHANGE_SCHEMA` constants, `CredentialChangeRequestV1` + `VerifiedCredentialChangeV1::verify`, `decide_credential_change` |
| `🌎️hub/🔐️auth/🧬️schema/🔣️.json` | `+$defs/CredentialChangeRequestV1` (see gap 10) |
| `🌎️hub/🔐️auth/🧬️schema/🟦️.ts` | route constants, `CredentialChangeRequestV1`, `credentialChangeRequestV1()` |
| `🌎️hub/🏗️bootstrap/🦀️.rs` | `post_auth_credential` + `journal_credential_change_refusal`; route mount; the credential route in `rate_limit_class`; `credential_command::dispatch` before `trusted_catalog_command` |
| `🌎️hub/📇️directory/🐘️postgres/🦀️.rs` | `append_credential_audit` + `set_password_credential`; explicit `Default` for the test-only `ArtifactGenesisTestControlV1` |
| `🌎️hub/📇️directory/🌐️neo4j/🦀️.rs` | the same pair, plus the same `Default` fix |
| `🌎️hub/📇️directory/{🪶️sqlite,🐘️postgres,🌐️neo4j}/🧪️tests/🔬️unit/🦀️.rs` | `credential_writes_and_facts_stay_in_one_transaction` (sqlite runs; the other two compile, gap 5) |
| `🌎️hub/🚀️local-bootstrap/🧬️schema/🔣️.json` | `publicSessionIssuance` is a documented boolean, not `const false` (§4.5) |
| `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts` | `LocalHubRun.publicSessionIssuance`; `localHubReadinessAdmitted` compares it against the run's own expectation |
| `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | `+local-relay-routing-check`, `+live-sign-in-check` |
| `🌎️hub/📦️packages/🦀️rust/📋️project.json` | the two targets (the live gate `cache: false`, `dependsOn: ["build-dev"]`) |
| `🌎️hub/🧫️fixtures/🧱️socket-grant-command-source/🔣️.json` | `rootImports` order corrected to the real import (gap 9) |
| `…/💻️os/🔨️modules/📇️directory/🔐️sign-in/🟦️.ts` | `HubSessionAuthorityV1`, `parseHubSessionAuthorityV1`, `runHubSessionAuthorityV1`, the two `me` constants |
| `…/🧱️elements/🔗️HubConnection/🟦️.tsx` | `HubSpaceMemberRowV1`, `readSpaceMembers` on the port, `parseHubSpaceMemberRowsV1` (member **window**), `authority`/`members`/`watchSpaceMembers` on the hook, the `me` follow-up and its re-arming deadline timer |
| `…/🧱️elements/🔗️HubConnection/🏛️workspace/🟦️.tsx` | `onlineUserIds` + `onSessionChange` replace the `members` prop; watches the active space |
| `…/🧱️elements/🏛️ShellHost/🟦️.tsx` | `hubBootstrapOriginV1()` (hub origin from `VITE_S_HUB_URL`), `openHubWorkspace` (overlay state, `/hub` only in host mode), `hubSessionPresence` state, `hubOnlineUserIds`, the `os.openHub` handler, the indicator's `onSignIn` |
| `…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | `OPEN_HUB_COMMAND_ID` + its `buildOsCommands` entry |
| `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx`, `…/🎯️targets/⚛️react/🟦️.tsx` | `ui.command.openHub`, en + de |
| `…/🧱️elements/🔐️HubSignIn/🧪️tests/🧩️component/🟦️.tsx` | a `me`-answering fake; 3 new laws; 2 updated for the new behaviour |
| `…/🧱️elements/🏘️SpaceBrowser/🧪️tests/🧩️component/🟦️.tsx` | `MEMBER_ROWS` fake + 2 new laws (roster/presence join, member-window decoder) |
| `.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json` | `⚖️gate🔐️hub-auth🤝️live-sign-in` + `⚖️gate🧭️local-relay📛️admission`, inserted identically in both |

### Ticket-folder artefacts

`🐍️au3-live-sign-in-probe.ts` (promoted to the hub gate), `🐍️au3-browser-probe.mjs`,
`🐍️au3-diagnose.mjs`, `📜️au3-serve.sh`; captures `🗑️generated/au3-*.txt` and
`au3-browser-{ada,bo}.png`.

## 9. Re-running the proofs

```bash
# self-provisioning hub gate (boots its own hub on a fresh data root)
cd 🌎️hub/📦️packages/🦀️rust && bun ./📜️script.ts live-sign-in-check
cd 🌎️hub/📦️packages/🦀️rust && bun ./📜️script.ts local-relay-routing-check

# browser proof: one hub held open, one activated React playground, then the probe
OS_HUB_CREDENTIAL_SIGN_IN=1 bun 🌎️hub/🔐️auth/🧪️tests/🤝️live-sign-in/🟦️.ts --hold   # prints origin + credentials
<ticket>/📜️au3-serve.sh animate 6081 http://127.0.0.1:8831                          # detached, nohup
bun <ticket>/🐍️au3-browser-probe.mjs http://127.0.0.1:6081 http://127.0.0.1:8831 \
  ada@example.org "correct horse battery staple" bo@example.org "another perfectly fine phrase"
```

**Environment note.** The shared cargo build dir had grown to 313 GB and the volume hit 0 bytes
free mid-slice; pruning incremental **sessions** older than 120 minutes
(`find … -name incremental -prune -exec find {} -mindepth 2 -maxdepth 2 -type d -mmin +120 -exec rm -rf {} +`)
freed **53 GiB** without cold-starting any peer's crate.
