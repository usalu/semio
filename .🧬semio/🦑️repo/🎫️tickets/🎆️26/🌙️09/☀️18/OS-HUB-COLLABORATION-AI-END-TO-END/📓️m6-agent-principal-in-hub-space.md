# 📓️ M6 — the AI agent as its own principal inside a hub space

Slice M6 (session 5 resume, 2026-09-20). Closes, or measures honestly, the intersection of
**outcome 3** (collaboration over the hub) and **outcome 4** (AI over the semio MCP): an agent
connected through `semio-os-mcp --hub` acts inside a hub space as a principal **distinct from the
human who delegated to it**.

Targets from `📓️g10-goal-gap-reaudit.md`:

- **O4-11** — "no production caller anywhere spawns `semio-os-mcp --hub` with a delegated credential —
  'AI agent as a collaborator in a hub space' has no entry point" (also M4 honest gap 5).
- **O4-14** — "hub directory has no 'AI agent principal' concept distinct from the delegating human —
  an agent's edit is indistinguishable from the human whose credential it borrowed at the presence
  layer".
- §D outcome 4 checkbox: "An MCP agent can edit inside a real hub space, replicate over the wire, and
  show up (by actor string at minimum) to a human collaborator".

Status legend: ✅ landed and executed · 🚧 landed and compiled, not executed · ❌ not done (see §9).

---

## 0. Inherited state

**Nothing.** The predecessor (launched ~01:05, cut ~01:15 by the desktop app restart) left no report,
no `🗑️generated/m6-*` capture and no source edit — verified by `ls 🗑️generated | grep m6` (empty),
`git status --short` over the os-mcp and hub auth paths (only peers' files modified), and the absence
of any `📓️m6-*.md`. Every line below is this session's.

## 1. Measured baseline — what existed before this slice

| fact | evidence |
|---|---|
| `--hub <url> --space <id>` parsed, but `server_for_workspace_options` refused without the **fd-3 local-bootstrap** credential | `🌉️mcp/🦀️.rs:726-742` → `claimed_local_hub_credential("mcp")`, minted only by a `dev s` launcher for its own children |
| that credential is structurally unreachable from an MCP client config | `decode_local_hub_credential` requires a length-prefixed frame on **fd 3** and pins `hubOrigin` to `http://127.0.0.1:<port>` (`📇️directory/🔌️client/🦀️.rs:690-700`). Claude Desktop / Claude Code / Cursor spawn a server with plain stdio and no spare descriptors — sibling D2 reached the same conclusion independently |
| the hub had **no** delegation concept at all | `grep -rn -i delegat 🌎️hub/📇️directory/🦀️.rs` → one hit, an unrelated doc comment |
| `AuthSessionKind` had exactly two variants, enforced by a sqlite `CHECK` | `📇️directory/🦀️.rs:135`, `🪶️sqlite/🦀️.rs:155` |
| presence had **no** principal-kind concept anywhere in the tree | `grep -rn "PresenceKind\|presence_kind\|presenceKind"` over `*.rs`/`*.ts`/`*.json` → **0 hits** |
| the hub already *overwrites* every identity field a client puts on a presence peer | `refresh_document_presence` (`🏗️bootstrap/🦀️.rs`) rebuilds `user_id`/`role`/`color`/`surface`/`label` from the admitted slot and keeps only the app-owned payload — the seam an admitted `principalKind` belongs in |
| a live hub is up on `127.0.0.1:7501` | sibling C1c's `🐍️c1c-hub-hold.ts` (pid 4409) holding `os-hub` pid 5468, confirmed by `lsof` |

## 2. Item 1 — a scoped, revocable agent credential, issued and revoked as hub events ✅

### 2.1 Why it is an `AuthSession`, not a new auth mechanism

The brief's constraint was "same event-sourced style as AU1's sessions; no new ad-hoc table
semantics". The design that satisfies it literally:

- the **delegation** is a capability record shaped exactly like `InviteRecord` — the repo's existing
  "scoped, revocable, one-time-token" pattern (selector + secret digest + expiry + revocation
  columns; raw capability bytes never retained in the read model or the log);
- the **agent session** is an ordinary `AuthSessionRecord`, minted by the same `issue_auth_session`
  (which already appends `session-issued`) and killed by the same `authorization_generation` bump
  that already invalidates socket grants and open document plans. **No hub route needed a special
  case to authenticate an agent.**

What is new is one enum variant, one capability kind, and one table.

### 2.2 The capability

`CapabilityKind::AgentDelegation` (`📇️directory/🦀️.rs`), prefix `delegation.v1`, digest domain
`semio/hub/agent-delegation/v1\0`, materialised through the existing `capability_type!` macro as
`AgentDelegationCapability`. It is deliberately **not** a `HubCapability` variant: a delegation is
never accepted anywhere a session or a share is, only at `POST /auth/agent-sessions`.

### 2.3 The record and its scope

`AgentDelegationRecord` (`📇️directory/🦀️.rs`, model module): `id`, `selector`, `secret_digest`,
`space_id`, `delegating_user_id`, `agent_label`, `audience`, `created_at`, `expires_at`,
`revoked_at`, `revoked_reason`.

Scope = **space + allowed capability audience**. `AgentAudience` is a closed two-value enum
(`read` | `edit`) whose `space_role()` is `Spectator` / `Author`. `SpaceRole` has no administrative
variant at all, so "an agent can never administer a space" holds by construction rather than by a
check someone can forget: the role vocabulary cannot spell it.

TTL bounds are deliberately tighter than a browser session's: 60 s … 90 days, default 7 days
(`MIN/MAX/DEFAULT_DELEGATION_TTL_SECS`). An agent *session* minted from a delegation lives 1 h
(`AGENT_SESSION_TTL_SECS`), so every re-exchange re-reads the delegation's revocation state.

### 2.4 Issuance and revocation as events

Sqlite (`🪶️sqlite/🦀️.rs`): one new table `hub_agent_delegation` (`CREATE TABLE IF NOT EXISTS`, so
additive on an existing data root) plus one index, and four trait methods:

| method | what it writes, in ONE transaction |
|---|---|
| `create_agent_delegation` | `INSERT hub_agent_delegation` **+** the `agent-delegated` fact |
| `revoke_agent_delegation` | `UPDATE … revoked_at` **+** revoke every agent session minted from it (`device_instance_id = <delegation id> AND session_kind='agent'`, generation bumped, one `session-revoked` fact each) **+** the `agent-delegation-revoked` fact |
| `list_agent_delegations` | read-only projection → `AgentDelegationRow`; no selector, no digest |
| `load_agent_delegation` | by selector only; the secret comparison is the pure law's job |

`hub_auth_audit.event_kind` is a free `TEXT` column, so the three new fact kinds
(`agent-delegated`, `agent-delegation-revoked`, `agent-session-issued`) needed **no DDL change** —
they land in the same log AU1's `credential-sign-in` does. The `hub_auth_session.session_kind` CHECK
gained `'agent'`.

Postgres and Neo4j get the file's established **erroring-default** trait bodies (AU1 §4's pattern),
so they compile unchanged and report the capability as unavailable rather than silently dropping a
delegation. Same honest gap AU1 carried; see §9.

### 2.5 The pure law

`agent_session_preflight(record, capability, requested_audience, now_ms)` → `Mint` / `Revoked` /
`Expired` / `Denied`. Three different wrong things — no such delegation, wrong secret, **and an
audience wider than the one delegated** — all collapse into `Denied`, so a delegation id cannot be
probed for existence. `decide_agent_session` (`🔐️auth/🤖️agent/🦀️.rs`) maps that to the
client-observable code; `Revoked`/`Expired` are reported only to a caller that already proved the
secret, because it needs to know whether to stop or to ask its human for a new credential.

### 2.6 The routes

New scope `semio_hub::auth::agent` (`🌎️hub/🔐️auth/🤖️agent/🦀️.rs`, ~300 lines) + four handlers in
`🏗️bootstrap/🦀️.rs`:

| route | bearer | effect |
|---|---|---|
| `POST /auth/agent-delegations` | the human's **session** (an agent session is refused `403 forbidden` — an agent can never delegate onward) | caller must be `SpaceRole::Author` of the target space; `201` with the token shown **once** |
| `GET /auth/agent-delegations?space=<id>` | same | this human's delegations in that space, revoked ones included; never a token or selector |
| `DELETE /auth/agent-delegations/{id}` | same | `204`; not-yours is `403`, never `404` |
| `POST /auth/agent-sessions` | the **delegation capability** | exchanges it for a `SessionCapability` whose record carries `session_kind = agent`, `identity_provider = agent.delegation.v1`, `device_instance_id = <delegation id>` |

All four are `no-store`, carry the `semio.hub.auth.agent-error/v1` body (seven codes, each with
exactly one status), and are charged against AU1's existing `RateLimitClassV1::Auth` bucket (per
delegating user on the administration routes, per delegation selector on the exchange).

## 3. Item 2 — `semio-os-mcp --hub` session mode ✅ (compiled and unit-tested; not run against a live hub)

### 3.1 The credential is a FILE PATH, never a secret in argv or env

New module `🌉️mcp/🤖️agent-credential/🦀️.rs`. Flags (both `stdio` and `http` modes):

- `--credential-file <path>` — **the shape a client config can express.** This is the one that
  matters: sibling D2's finding, confirmed here, is that no MCP client can pass an fd-3 envelope, so
  before this flag `--hub` was mechanically unusable for an end user.
- `--credential-fd <n>` — the stdin-handle shape for a launcher that would rather pipe it. `3` is
  always refused (the local-bootstrap envelope) and `0/1/2` are refused in `stdio` mode (this
  process's own MCP framing).

Argv carries only a *location*. `AgentCredentialV1`'s `Debug` prints `<redacted>`, its `Drop` wipes
the token, the read buffer is zeroed, and M1's process-entry seal is untouched — no new environment
carrier was introduced, so `protected_credential_environment_is_absent()` still holds.

### 3.2 What the file must be

```json
{ "schema": "semio.hub.agent-credential/v1", "hubOrigin": "…", "spaceId": "…",
  "audience": "read|edit", "token": "delegation.v1.<32 hex>.<64 hex>" }
```

Checked at start-up, so a bad credential is a typed refusal before the first tool call rather than
an opaque `401` later: **mode `0600`** on unix (group- or world-readable is refused, and the message
names `chmod 600`), regular file, ≤ 16 KiB, exact schema, audience in the enum, exact token shape,
and `hubOrigin`/`spaceId` matching the `--hub`/`--space` that were passed.

### 3.3 The exchange

`workspace::remote::exchange_agent_session` (`🏠️workspace/🔗️remote/🦀️.rs`) POSTs
`/auth/agent-sessions` over a short-lived `NativeDirectoryTransport` — the only network call this
process makes before it has a session — and turns the result into a `LocalHubCredential` through the
new `LocalHubCredential::adopt_session_capability` (`📇️directory/🔌️client/🦀️.rs`), which holds a
network-obtained capability under the same redacting `Debug`, constant-time comparison and
zero-on-drop discipline as an inherited one, and deliberately does **not** pin the origin to
loopback. `NativeHubBindingDriver::connect` and everything below it is unchanged.

A refusal is decoded into a typed `GatewayError` naming the hub's own code, so an operator reads
`the hub refused this agent delegation: delegation-revoked` rather than `401`; `rate-limited` and
`directory-unavailable` are marked retryable, the rest are not.

`README.md`'s `--hub` section (which D2 had written ahead of the implementation, and whose flag names
this implementation matches exactly) gained the credential-file shape, where it comes from, and what
is validated.

## 4. Item 3 — attribution and per-user undo ❌ (design landed, enforcement not)

What exists: `agent_principal_id(delegation_id) = "agent:<delegation id>"` and
`is_agent_principal(actor)` in `🔐️auth/🤖️agent/🦀️.rs`, stable across the agent's sessions and across
hub restarts, structurally impossible to confuse with a human's `user:<id>#<session>`. It is returned
by both the creation receipt and the session exchange, and it is what the roster badge keys on.

What does **not** yet exist, honestly: the hub mints a socket `actor_id` as an opaque
`hub.v1.<sha256>` (`socket_actor_id`), and replicated edits are keyed by *that*, not by the agent
principal id. So an agent's edits are already **a different actor** from the delegating human's
browser session — which is what makes per-actor undo treat them separately — but the actor string a
collaborator sees is the opaque socket actor, not `agent:<id>`. Carrying the principal id into
`socket_actor_id`'s derivation (or alongside it) is the remaining step, and I did not take it: it
touches the actor-material derivation that every existing socket test pins.

**So: distinct actor ✅ (by construction, via a distinct session), distinct *legible* actor ❌.**

## 5. Item 4 — the `agent` presence kind ✅ (wire + hub), ❌ (roster UI)

### 5.1 Wire — both codecs

`PresencePeer` (`🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs`) gained
`principal_kind: Option<PresencePrincipalKind>` (`Human` | `Agent`) in **both** codecs:

- **binary** — flag **bit 11**, one declaration-order tag byte. The decoder's unknown-flag guard
  moved from `flags >> 11` to `flags >> 12`, and `PresencePeerReader::principal_kind` rejects a tag
  outside the enum rather than defaulting. A peer that leaves the bit unset is the pre-agent wire
  shape and round-trips byte-for-byte as `None`.
- **DslValue/JSON** — key `principalKind`, the camelCase `wire_name()` string; an unknown spelling is
  a typed `ValueError` under `principalKind`, never a silent `Human`.

`None` means "never declared" and a reader treats it as `Human` — written into the field's doc
comment so no reader invents a third state.

### 5.2 Server-authoritative, never client-claimed

`PresenceLeaseSlot` (`🏗️bootstrap/🦀️.rs`) gained `principal_kind`, filled at socket admission from
the **authenticated** session's `AuthSessionKind` — threaded
`SocketSubjectV1::Session { …, session_kind }` → `AuthOutcome::Session { …, session_kind }` → the
slot. `refresh_document_presence` stamps `principal_kind: Some(slot.principal_kind)` onto every
outbound roster entry in the same normalization step that already overwrites `user_id`/`role`/
`color`/`surface`. A human session therefore cannot claim to be an agent, and an agent cannot hide
as a human.

### 5.3 TypeScript twin and roster badge ✅

- **TS codec twin** (`🧰️framework/🔨️modules/📡️replication/🟦️.ts`): `ArtifactPresencePeer.principalKind`,
  `PRESENCE_PRINCIPAL_KINDS`/`ArtifactPresencePrincipalKind`, bit 11 in `encodePresencePeer`, the
  unknown-flag guard widened `0x7ff` → `0xfff`, and `PresencePeerReader.principalKind()` which
  refuses an out-of-enum tag by name. Byte-for-byte the Rust codec's twin.
- **Shared vectors** (§7.2) are read by both sides, so the two codecs cannot drift silently.
- **Projection** (`🏛️ShellHost/👥️presence-scope/🟦️.ts`): `principalKind === "agent"` → `isAgent: true`
  on the ui-react `PresencePeer`. Absent and `"human"` are the same thing.
- **Badge** (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🟦️.tsx`): `PresencePeer.isAgent`, a
  `data-presence-kind="agent"|"human"` attribute on every row, and a small robot marker. The badge is
  `aria-hidden` **and** its word is folded into the row's own `aria-label`/`title`, so a screen reader
  hears `Drafting agent (AI agent, Editing)` rather than a decorative glyph a11y cannot see.
- **en + de**: `ui.presence.kind.agent` added to the `UiTranslationSchema` and to both catalogues
  (`AI agent` / `KI-Agent`, with beginner tiers "An AI agent someone gave access to" /
  "Ein KI-Agent, dem jemand Zugriff erteilt hat").

## 6. Item 5 — delegation UI in the hub workspace ❌

Not started — the one brief item with no code at all. The routes it would call are live and shaped for it (`AgentDelegationSummaryV1` is
exactly a list row; the receipt is exactly a one-time download). `HubWorkspace` is reachable from the
shell as of AU3 §4.1, which is where the panel belongs. The wgpu parity debt this creates is noted
for the second coordinator's WG6: nothing on the wgpu side reads `principal_kind` either.

## 7. Tests and real counts

### 7.1 Compile gates — all executed, all clean

| crate | command | result | capture |
|---|---|---|---|
| `semio-framework-replication` | `cargo check -p … --lib` | **0 errors**, 7.83 s | `m6-replication-check-1.txt` |
| `semio-framework-replication` | `cargo check -p … --all-targets` | **0 errors**, 9.82 s | `m6-replication-check-2.txt` |
| `semio-framework-plugin` (dependent) | `cargo check -p … --all-targets` | **0 errors** (warnings only), 5 m 42 s | `m6-plugin-check-1.txt` |
| `semio-hub` | `cargo check -p … --lib` | **0 errors** | `m6-check-round5.txt` |
| `semio-framework-os-mcp` | `cargo check -p … --all-targets` | **0 errors** | `m6-mcp-check-3.txt` |

**Coordinator's red-crate report, answered:** `semio-framework-replication` compiles, `--lib` and
`--all-targets`, and so does `semio-framework-plugin` which depends on it. The window C1c saw was
between adding the `PresencePeer` field and patching its eleven construction sites; all eleven are
patched (two one-liners and six multi-line literals in the replication/plugin/store/renderer test
fixtures, the wgpu shell's heartbeat, and the hub's own normalization).

### 7.2 New unit laws — EXECUTED (rule 25's private uplift dir unblocked them)

`CARGO_TARGET_DIR=…/⚡️cache/cargo/target-m6` (rule 25) ended the starvation that cost four earlier
attempts. Capture: `m6-tests-uplift.txt`, `m6-tests-uplift-2.txt`.

| suite | command | result |
|---|---|---|
| `semio_framework_os_mcp::agent_credential` | `cargo test -p semio-framework-os-mcp --lib agent_credential` | **7 passed / 0 failed** (403 filtered out) ✅ |
| presence codec (replication) | `cargo test -p semio-framework-replication --lib presence` | first run **16 passed / 1 FAILED** — see below; fixed and rerun |
| `semio_hub::auth::agent` | `cargo test -p semio-hub --lib auth::agent` | rerun in flight at hand-off (first attempt was blocked by a peer's transient red `semio-framework-ui`, since fixed by that peer) |

**What the failing test found — a real defect in my change, not a stale fixture.**
`presence_peer_rejects_unknown_flag_bits` hand-built a peer with flag bit 11 set and asserted
`Malformed { what: "presence peer flags" }`. Bit 11 is exactly the bit I took for `principal_kind`,
so the law that froze the wire at 0..=10 fired precisely as designed. Fixed at the root
(`🔬️presence-codec/🦀️.rs`): the unknown-flag probe moved to **bit 12**, and the boundary is now
pinned from both sides — bit 11 with no body is refused as `"presence peer principal kind"`
(truncated), and an out-of-enum tag `2` is refused by the same name, so a future field taking bit 12
cannot land without moving this test with it.

Two new laws were added in the same file:
`presence_peer_principal_kind_round_trips_in_both_codecs` round-trips `None` / `Human` / `Agent`
through the binary codec **and** the `ToValue`/`FromValue` JSON codec, asserts the JSON spelling is
`"agent"`, and asserts an unknown `principalKind` string is a typed refusal rather than a silent
`human`.

**Shared cross-language vectors ✅.** `🧫️fixtures/👥️presence-peer-codec-v1/🔣️.json` — the corpus both
the Rust decoder test and the TS twin read — went from 27 to **31 cases**: `principal-kind-human`,
`principal-kind-agent` (both accepted, with canonical bytes and expected semantics),
`principal-kind-unknown-tag` and `principal-kind-truncated` (both rejected). The pre-existing
`unknown-flag` case was moved from bit 11 to bit 12 for the same reason as the Rust law.

The 9 hub laws: create-request bounds are exactly the schema's; `deny_unknown_fields` on both bodies;
the `agentInstanceId` charset equals `deviceInstanceId`'s; an agent principal is structurally
distinct from the delegating human; a live delegation mints and **a wrong delegation, a wrong secret
and a widened audience are one indistinguishable refusal**; an agent can never widen its own audience
at exchange time; revocation and expiry are reported only to a caller that proved the secret; no
audience exceeds `Author`; every error code has exactly one status and one wire spelling; and a
listing row serializes with neither the token nor the selector in it.

The 7 MCP laws (all green): a well-formed credential decodes and its `Debug` never prints the token;
eight malformed shapes (wrong schema, empty origin/space, unknown audience, a *session* token in
place of a delegation, truncated, uppercase hex, non-string) are each refused; the token shape is
exactly what the hub mints, one byte either way refused; **a 0644 or 0640 credential file is refused
with `chmod 600` in the message and a 0600 one is admitted**; the exchange body is the hub's own
schema; a refusal body never decodes as a grant; every hub refusal becomes a typed error naming the
hub's code with the right retryability.

### 7.2b Peer breakage encountered (neither caused nor fixed by me)

- `semio-framework-ui` was red for ~30 min (`E0609` ×4 in `🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs`), which
  blocked the hub test build. A later `cargo check -p semio-framework-ui --lib` returned **0
  errors** — the owning peer fixed it.
- `cargo check -p semio-hub --all-targets` currently reports exactly **one** error, and it is not
  mine: `StartupCatalogControl::new` in
  `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/📤️command/🦀️.rs:42` (`E0423`, struct used as a
  value; file mtime 08:02, a live peer edit). **All seven errors P4 and OB1r attributed to my
  in-flight symbols are fixed** — see §7.1b.

### 7.1b `cargo check -p semio-hub --all-targets`, answering P4/OB1r

P4 and OB1r were right: the `--lib` target was green but the `os-hub` **bin** and **bin test**
targets were not, because `prepare_agent_delegation` was `pub(crate)` (invisible to the bin crate)
and five construction/match sites had not been widened. All fixed:

| site | fix |
|---|---|
| `📇️directory/🦀️.rs` `prepare_agent_delegation` | `pub(crate)` → `pub` |
| `🏗️bootstrap/🦀️.rs:892` `SocketSubjectV1::revalidate` | pattern gained `..` |
| `🏗️bootstrap/🦀️.rs:7097` `AuthSessionKind` → `DirectorySessionKindV1` | added the `Agent` arm |
| `📇️directory/🧬️schema/🪪️session-authority-v1/🦀️.rs` + its `🧬️.schema.json` | `DirectorySessionKindV1::Agent` and `"agent"` in the enum — so `GET /auth/sessions/me` and `context_resolve` report an agent session as one |
| `🧪️tests/🔬️bin-unit/🦀️.rs` ×5 | four `SocketSubjectV1` literals + the `PresenceLeaseSlot` fixture |

Capture: `m6-hub-alltargets-1.txt` (10 errors, before) → `m6-hub-alltargets-2.txt` (only the peer's
`StartupCatalogControl` error left) → **`m6-hub-alltargets-3.txt`: 0 errors**, run after the schema
`$defs` and the drift law landed and after that peer fixed their file. `cargo check -p semio-hub
--all-targets` is green.

### 7.3 TypeScript — EXECUTED ✅

`bun ./📜️script.ts scoped-presence-check` in
`📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript`:
**9 passed / 0 failed** (16.27 s), plus the `scoped-presence-oracle` at `checks=29 clean`. The suite
was 7 laws before this slice; the two added are:

- *projects an agent peer as its own principal and badges it distinctly from the delegating human* —
  an agent and its delegating human appear as **two** roster rows, the agent carries `isAgent`, and a
  client that spells `principalKind: "human"` cannot acquire the badge;
- *renders the agent badge inside the accessible name, in en and de* — drives `uiI18n.changeLanguage`
  for both locales and asserts `AI agent` / `KI-Agent` inside the row's `aria-label`, the
  `data-presence-kind` attribute on both rows, and that a person never gets the badge element.

Capture: `m6-scoped-presence-vitest.txt`.

### 7.4 Schema authority for the agent scope ✅ (gap 9 closed)

`🌎️hub/🔐️auth/🧬️schema/🔣️.json` gained **11 `$defs`** — `AgentDelegationCapabilityV1`,
`AgentAudienceV1`, `AgentErrorCodeV1`, `AgentErrorV1`, `CreateAgentDelegationRequestV1`,
`AgentDelegationReceiptV1`, `AgentDelegationSummaryV1`, `AgentDelegationListV1`,
`AgentSessionRequestV1`, `AgentSessionMintResponseV1`, `AgentCredentialFileV1` — and
`AuthDurableEventKindV1` grew from four kinds to seven.

A new law, `the_agent_decoders_and_the_json_schema_admit_exactly_the_same_wire`, compiles each
`$def` through the same owned draft-07 validator AU1's law uses and runs every accepted and rejected
payload through **both** the schema and the Rust decoder: seven malformed create bodies, three
malformed session bodies, the receipt, the listing, the error body, a session capability rejected as
a delegation, all three new durable event kinds, and the MCP's own credential-file shape. So the
`$defs` are authority, not documentation, and the credential file the MCP reads is the same dialect
the hub speaks.

## 8. Live proof ❌ — not attempted, and here is exactly what a successor needs

Nothing in §2–§6 has been observed against a running hub. Two things made it impossible inside my
budget, and neither is a property of the code:

1. Rule 26 forbids me building `-p semio-hub`, so I cannot produce a binary carrying my routes. The
   coordinator's shared binary at
   `.🧬semio/🦑️repo/⚡️cache/cargo/target-coordinator-hub/debug/os-hub` is the intended source; at my
   hand-off I had not copied and booted it.
2. C1c's live hub on `127.0.0.1:7501` runs a binary built **before** this slice, so it has no
   `/auth/agent-delegations` route at all — curling it would prove nothing, and it is not mine to
   restart.

The proof a successor should run, in order (own data root, free port, never C1c's 7501):

```
cp .🧬semio/🦑️repo/⚡️cache/cargo/target-coordinator-hub/debug/os-hub <scratch>/os-hub
OS_HUB_CREDENTIAL_SIGN_IN=true OS_HUB_DATA=<fresh dir> <scratch>/os-hub --port <free>
# 1. sign in as a human, create a space, be its author
# 2. POST /auth/agent-delegations           -> 201, token shown once
# 3. write the token into a 0600 credential file (schema semio.hub.agent-credential/v1)
# 4. semio-os-mcp stdio --hub http://127.0.0.1:<free> --space <id> --credential-file <path>
#    -> stderr prints `acting as agent principal agent:<id> ("<label>") in space <id>`
# 5. context_resolve                        -> the agent's own session, sessionKind "agent"
# 6. open a directory socket as the human   -> the agent's roster row carries principalKind "agent"
# 7. DELETE /auth/agent-delegations/{id}    -> 204; the agent's next frame closes 4401
```

Steps 2, 4 and 7 are the three that have never run. Everything they exercise is unit-tested
(§7.2/§7.4) and compiles (§7.1), which is strictly weaker than having seen it work.

## 9. Honest gaps

1. **No hub integration test.** delegate → agent session → use → revoke → refused (+ cascade) as a
   `--bin os-hub` law, in the style of AU1 §5.1's seven, is **not written**. The four routes are in
   place and the revocation cascade is one transaction, so the test is mechanical — but nothing has
   exercised these routes end to end.
2. **My hub unit laws' verdicts are still unseen.** Rule 26 forbids me running `cargo test -p
   semio-hub`. The coordinator's 09:17 run (318 tests, 260/58) contains **no `auth::agent` line at
   all** — neither pass nor fail — and its capture is partial (46 PASS + 72 FAIL lines for 318
   tests), so it does not tell me anything about them. **Needs a hub rerun**, and I have changed hub
   source since 09:17 (the 11 `$defs`, the drift law, `prepare_agent_delegation` → `pub`,
   `DirectorySessionKindV1::Agent`).
3. **No live proof** (§8).
4. **No delegation UI** (§6) — the brief item with no code at all.
5. **The agent's legible actor string is not wired** (§4). Its *session* is distinct, so per-actor
   undo already separates it and the roster badge already distinguishes it, but the actor string a
   collaborator's history shows is the opaque hub-minted `hub.v1.<sha256>` socket actor rather than
   `agent:<delegation id>`. Carrying the principal id into `socket_actor_id`'s derivation touches
   material that every existing socket test pins, and I did not take that risk with my budget.
6. **Postgres and Neo4j cannot store a delegation** — erroring defaults, exactly like AU1's
   credential methods. Neither backend has ever been compiled (G2 §2).
7. **`AGENT_SESSION_ISSUED_EVENT` is journalled by the handler, not inside the mint transaction.**
   `issue_auth_session` writes `session-issued` transactionally; the agent-specific annotation is a
   second, best-effort append. A crash between them loses only the annotation, never the session —
   but it is not the one-transaction discipline §2.4 holds for issuance and revocation.
8. **The delegation `space_id` is not re-checked at exchange time.** It is checked at creation
   (author of that space) and the MCP refuses a credential whose `spaceId` differs from `--space`,
   but the hub does not re-verify at `POST /auth/agent-sessions` that the delegating user is *still*
   an author. A demoted human's agent keeps editing until the delegation expires or is revoked.
9. **Rate-limit policy for the agent routes reuses `RateLimitClassV1::Auth`** (10 burst, 1 per 6 s),
   sized for interactive sign-in. An agent re-exchanging hourly is far under it; nothing measured.
10. **The wgpu side reads none of this** — no `principal_kind` consumer in the native shell. Noted
    for the second coordinator's WG6.

## 10. Files changed

**New**
- `🌎️hub/🔐️auth/🤖️agent/🦀️.rs` — the `hub.auth.agent` scope (wire types, principal id, pure decision)
- `🌎️hub/🔐️auth/🧪️tests/🤖️agent-unit/🦀️.rs` — 9 laws
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🤖️agent-credential/🦀️.rs` — credential file/fd reader, exchange request/response codecs
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🤖️agent-credential/🦀️.rs` — 6 laws

**Modified**
- `🌎️hub/📇️directory/🦀️.rs` — `AuthSessionKind::Agent`, `CapabilityKind::AgentDelegation` + `AgentDelegationCapability`, `AgentDelegationRecord`/`AgentAudience`/`IssuedAgentDelegation`/`AgentDelegationRow`, `prepare_agent_delegation`, `agent_session_preflight`, four trait methods with erroring defaults + their `HubDirectories` forwards, the three event-kind constants, `AGENT_IDENTITY_PROVIDER`, `AGENT_LABEL_MAX_BYTES`, `AGENT_DELEGATION_PAGE_MAX`
- `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs` — `hub_agent_delegation` table + index, the `session_kind` CHECK, the four method bodies, `agent_delegation_row`
- `🌎️hub/🔐️auth/🦀️.rs` — mounts `pub mod agent`
- `🌎️hub/🏗️bootstrap/🦀️.rs` — four handlers + `agent_error_response` + `delegating_principal` + `journal_agent_fact`, three routes, `SocketSubjectV1::Session.session_kind`, `AuthOutcome::Session.session_kind`, `PresenceLeaseSlot.principal_kind`, the presence normalization stamp
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — seven `SocketSubjectV1::Session` construction sites
- `🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs` — `PresencePrincipalKind`, `PresencePeer.principal_kind`, both codecs, the widened flag guard, `PresencePeerReader::principal_kind`
- `🧰️framework/🔨️modules/📡️replication/📡️wire/🧪️tests/🔬️presence-codec/🦀️.rs` — six construction sites
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs` — `LocalHubCredential::adopt_session_capability`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🦀️.rs` — `HubOptions.credential`, `AgentCredentialSource`, the agent branch of `server_for_workspace_options`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/🦀️.rs` — mounts `pub mod agent_credential`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs` — `--credential-file` / `--credential-fd` in both modes, `parse_credential_fd`, the widened `HubArgs`, the usage line
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs` — `exchange_agent_session`, `agent_instance_id`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧪️tests/🔬️bin-quick/🦀️.rs` — two `HubOptions` sites + one new argv law
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/README.md` — the credential-file shape and its validation
- three test fixtures that construct `PresencePeer`: `🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`, `🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs`, `📺️renderer/…/🐚️Shell/🧪️tests/🔬️wgpu-identity-directory-presence/🦀️.rs`, and the wgpu shell's own heartbeat in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`

**Also modified (second half of the session)**
- `🌎️hub/🔐️auth/🧬️schema/🔣️.json` — 11 new `$defs`, `AuthDurableEventKindV1` 4 → 7 kinds
- `🌎️hub/🔐️auth/🧪️tests/🤖️agent-unit/🦀️.rs` — the schema-drift law (10th law)
- `🧰️framework/🔨️modules/📡️replication/🟦️.ts` — the TS codec twin of `principalKind` (both directions)
- `🧰️framework/🔨️modules/📡️replication/🧫️fixtures/👥️presence-peer-codec-v1/🔣️.json` — 27 → 31 shared vectors
- `🧰️framework/🔨️modules/📡️replication/📡️wire/🧪️tests/🔬️presence-codec/🦀️.rs` — unknown-flag law moved to bit 12, bit-11 boundary pinned from both sides, `principal_kind` round-trip law in both codecs
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🟦️.tsx` — `isAgent`, `data-presence-kind`, the badge, the accessible name
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx` + `🎯️targets/⚛️react/🟦️.tsx` — `ui.presence.kind.agent` (en + de, both tiers)
- `🏛️ShellHost/👥️presence-scope/🟦️.ts` — projects `principalKind` → `isAgent`
- `📺️renderer/🧑‍🎨engine/🧪️tests/👥️scoped-presence/🟦️.tsx` — two vitest laws
- `📇️directory/🧬️schema/🪪️session-authority-v1/🦀️.rs` + `🧬️.schema.json` — `DirectorySessionKindV1::Agent`
- `🌎️hub/📇️directory/🦀️.rs` — `prepare_agent_delegation` `pub(crate)` → `pub` (the `os-hub` bin needs it)

**Ticket artefacts:** this report plus `🗑️generated/m6-*.txt` (five compile captures, one test
capture).

No `📜️script.ts` / `📋️project.json` / `.vscode/launch.json` row was added: this slice introduces no
new runnable command — everything runs under the three crates' existing cargo targets.

---

# M6b — delegation UI, legible actor, integration test, live proof

Successor slice to M6 (session 5c, 2026-09-20, started 11:45). Inherited: **nothing of its own** —
`ls 🗑️generated | grep m6b` empty, no `📓️m6b-*.md`, no M6b edit in `git status`. M6's four open
items (§9 gaps 1, 3, 4, 5) are this slice, in this order: delegation UI → legible actor → hub
integration test → live proof.

## M6b.0 Status board

| item | state |
|---|---|
| 1. Delegation UI in the hub workspace | ✅ landed, **86/86 vitest green** (66 before → 20 new laws) |
| 2. Legible `agent:<name>` actor in history/rosters | ✅ landed (hub + TS), compile-proven; the hub half is **not** executed (needs a document socket, §M6b.4.3) |
| 3. Hub integration test (delegate → session → use → revoke → refused) | ✅ 3 laws written, `cargo check -p semio-hub --all-targets` **0 errors** — **needs hub rerun** |
| 4. Live proof, M6 §8's seven steps | ✅ **6 of 7 observed on a real hub**; two live-only defects found, fixed and re-observed; step 6 not observed |

## M6b.1 Delegation UI ✅ — M6 §9 gap 4 closed

The pane M6 §6 left with no code at all. Three files, mounted in `HubWorkspace` below `🏘️SpaceBrowser`.

### 1.1 The pure contract first

`🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🤖️delegations/🟦️.ts` (new, ~280 lines) — the
`🏘️spaces/🟦️.ts` twin for delegations. It carries the hub's **own** bounds
(`AGENT_LABEL_MAX_BYTES` 128, TTL 60 s … 90 d, `AGENT_DELEGATION_PAGE_MAX` 256) so a malformed
request is refused in the browser with a cause instead of as an opaque `400`, and it renders the
credential file in the exact five-key shape `semio-os-mcp --credential-file` decodes.

| export | what it is |
|---|---|
| `createAgentDelegationBodyV1` | the exact `POST` body, in the schema's key order, bounds-checked |
| `parseAgentDelegationReceiptV1` | the 201 receipt; refuses a body whose `token` is not a delegation capability, and one whose `agentPrincipalId` is not `agent:<its own id>` |
| `parseAgentDelegationListV1` | drops a corrupt row rather than failing the listing |
| `agentDelegationRowsV1` / `agentDelegationStateV1` | live → expired → revoked, newest first, locale-independent; `expired` is derived from a clock, never stored |
| `agentCredentialFileV1` / `agentCredentialCommandV1` | the downloadable file, and the command line that uses it |
| `agentDelegationErrorFromResponseV1` | the hub's own `semio.hub.auth.agent-error/v1` code, falling back to the status |
| `actorDisplayV1` / `actorDisplayFromRosterV1` | item 2's legible actor (§M6b.2) |

### 1.2 The lane and the pane

- `🔗️HubConnection/🟦️.tsx` — `HubConnectionPortV1` gained three required methods
  (`listAgentDelegations` / `createAgentDelegation` / `revokeAgentDelegation`) and one optional
  `saveFile`; the hook gained `delegations` / `delegationPhase` / `delegationError` /
  `agentCredential` and five actions, each with its own `AbortController` and the existing
  connection-generation guard, cleared on connection switch, forget and sign-out exactly as the
  invitation lane is. `AgentDelegationRefusalV1` carries the hub's code across the port boundary.
  The fetch adapter reads each body **once** (a `Response.text()` called twice throws).
- `🤖️AgentDelegations/🟦️.tsx` (new) — purely presentational. Create form (name, audience, expiry),
  the one-time credential block, the listing, and a **two-step inline withdrawal**: the first press
  opens a `role="group"` naming the agent and stating that its open sessions end at once, and only
  the control inside that group revokes. Deliberately not `window.confirm`: a shell that owns its own
  focus cannot hand it to the browser, and a screen reader must hear the cost before the control.
- `HubWorkspace` mounts it with `canDelegate = spaceRowInvitableV1(active row)` — the same
  author-only predicate the hub enforces, so a spectator is told why rather than shown a form the hub
  will refuse. D2's first-run tour and AU2's two panes were re-read immediately before the edit and
  are untouched except for the one added element and the header sentence.
- `ShellHost` supplies `saveFile` through the **existing** `downloadMediaExport` helper — no new
  download machinery.

### 1.3 The one-time secret never leaks into the page

The token is turned into a file **inside the hook**, once; the pane receives only the rendered file,
and the raw token is rendered **only** when the browser refused to save it (the escape hatch a human
needs to not lose a credential they can never fetch again). A law asserts the token string is absent
from the DOM on the healthy path.

### 1.4 en + de, a11y, phone width, stories, laws — all executed

- both catalogues gained 42 `os.hub.agent.*` keys in both tiers (`registerUiTranslationBundles` makes
  a key added to one locale and not the other a compile error);
- `📖️stories/🧪️.story.tsx` — 10 stories (default, first run, loading, credential-shown-once,
  download-refused, spectator, no-space, hub-refused, signed-out, phone), matching
  `🏘️SpaceBrowser`'s story file which is the neighbouring pane's convention;
- `🧪️tests/🧩️component/🟦️.tsx` — **20 laws**, registered in the react target's
  `engineTestSuites` (`elementSuite("🤖️AgentDelegations", …)`) **and** in
  `hub-sign-in-spaces-check`'s file list. Registering only the second would have been a gate reading
  green while measuring nothing: the first run of the verb reported `Test Files 2 passed` because the
  config's include list is explicit. Capture `m6b-hub-ui-vitest-1.txt` is exactly that blind run.

| run | result | capture |
|---|---|---|
| `hub-sign-in-spaces-check` before registering in the config | 66 passed — **new file silently not run** | `m6b-hub-ui-vitest-1.txt` |
| after registering, first attempt | 81 passed / **5 failed** | `m6b-hub-ui-vitest-2.txt` |
| after fixing the five | **86 passed / 0 failed** (3 files, 11.4 s) | `m6b-hub-ui-vitest-3.txt` |

The five failures were real and worth naming: two were mine using `screen.queryByRole` /
`screen.getByLabelText`, which the repo's **owned** `UiTestScreen` (`🖱️ui/🎯️targets/⚛️react/🖌️render`)
does not have — it exposes only `getAllByRole`/`getByRole`/`getByText`/`queryByText`, so an "is not
offered" law now reads accessible names off the tree; one was my fixture asserting on the wrong row
because `agentDelegationRowsV1` had correctly sorted the newer delegation first; one was a German law
asserting `chmod 600` on a pane rendered without a credential (the warning lives in the credential
block — the law now renders one, so the permission sentence is pinned in **both** locales).

**wgpu parity debt (recorded, not paid):** the native shell has no delegation surface. `🔗️HubConnection`,
`🔐️HubSignIn` and `🏘️SpaceBrowser` each have a `🎯️targets/🧊️wgpu/🦀️.rs` twin; `🤖️AgentDelegations` has
none, and nothing on the wgpu side reads `principal_kind` either (M6 gap 10). A wgpu user can neither
create nor revoke a delegation. For the second coordinator's WG6.

## M6b.2 Legible actor ✅ — M6 §9 gap 5 closed by construction, not by touching actor material

M6 declined this because "carrying the principal id into `socket_actor_id`'s derivation touches
material that every existing socket test pins". That framing was the trap: the brief does not ask for
a different actor **id**, it asks for a different actor **name**. Those are separable, and separating
them is also the only version that keeps per-actor undo correct.

### 2.1 What stays exactly as it was

`socket_actor_id` is untouched. The hub keeps minting an opaque, per-session `hub.v1.<sha256>`; an
agent's socket is a different session from its human's, therefore a different actor, therefore
per-actor undo already separates their edits. Nothing that any socket test pins moved.

### 2.2 What was actually wrong

`PresenceLeaseSlot.label` was `authenticated_user.display_name` — and an agent session's `user_id`
**is the delegating human's**. So an agent's roster row was not merely opaque, it was **wrong**: it
read as the human. Likewise `record_sync_session_open`'s `client_label` was passed `&actor.0`, the
opaque id, for every connection.

### 2.3 The fix, hub side

- `SocketSubjectV1::Session` gained `device_instance_id` (3 construction sites in `🏗️bootstrap`,
  11 in `🧪️tests/🔬️bin-unit`). For an agent session that field **is the delegation id** — that is
  already how `revoke_agent_delegation` finds the sessions it cascades over, so no new coupling.
- `agent_delegation_for_session` (`🏗️bootstrap/🦀️.rs`) resolves the delegation once at socket
  admission through the **existing** `list_agent_delegations(space, delegating_user, page_max)`. No
  new trait method and therefore no new erroring default in the two uncompiled backends: a
  delegation always belongs to the space and the human its session names, so that human's own
  listing for that space is the smallest read that answers the question.
- `PresenceLeaseSlot.label` = the delegation's `agent_label`, falling back to the human's display
  name. `refresh_document_presence` already stamps the slot's label onto every outbound roster entry,
  so an agent peer now travels as `label: "Drafting agent"`, `principalKind: "agent"` — and a client
  that spells either itself is still overwritten.
- `client_label` = `agent_actor_display(label)` = `agent:<label>` for an agent, unchanged `actor.0`
  for a human. So the hub's own connection log and admin connections page name the agent.

### 2.4 The fix, client side

`actorDisplayV1(actor, label, isAgent)` and `actorDisplayFromRosterV1(actor, roster)` in
`📇️directory/🤖️delegations/🟦️.ts`. An agent renders as `agent:<its label>`, a person as their display
name, and the opaque id is shown only when there is no name at all — an unlabelled actor is still
better named by its id than by a guess. `actorDisplayFromRosterV1` is the form a surface with an
actor string and no peer (history, attribution, a conflict notice) needs. Both are covered by the
law *names an agent by its delegation label while leaving the actor identity untouched*, which
asserts on both halves: the display is `agent:Drafting agent` **and** the actor argument is returned
unchanged.

### 2.5 What is proven and what is not

Compile-proven (`cargo check -p semio-hub --all-targets`, **0 errors**) and, on the TS side,
executed. The hub half is **not** executed: an agent presence row requires a WebSocket document
socket, which requires an announced document in a data root whose `artifactAuthority` gate is open
(§M6b.4 step 6). The `principal_kind` stamping it builds on is M6's, and that is unit-tested.

## M6b.3 Hub integration test ✅ written — M6 §9 gap 1 — **needs hub rerun**

Three `#[tokio::test]` laws in `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` (region `🔖️AgentDelegation`), in the
hub's own integration layout — the `os-hub` **bin** target, on a real `TcpListener` through
`spawn_server`, next to AU1's seven credential-sign-in laws. Every step is one HTTP call; nothing
reaches into the directory to construct a state a client could not have reached.

| law | what it pins |
|---|---|
| `an_agent_delegation_mints_a_session_that_works_until_it_is_revoked` | the whole lifecycle: `201` + one-time token + `agent:<id>` receipt → listing (no token, `lastUsedAtMs` null) → exchange → `sessionKind: "agent"` on `/auth/sessions/me` under the human's membership → an agent cannot delegate onward (`403`) → the listing now reports a last use → revoke `204` → **the already-minted session is `401`** (the cascade) → re-exchange is `403 delegation-revoked` → the withdrawn row stays visible, marked → all four durable facts (`agent-delegated`, `agent-session-issued` with `peer_class: "agent"`, `agent-delegation-revoked`, `session-revoked`) are in the same log credential sign-in writes |
| `an_agent_can_never_widen_its_own_audience_or_probe_another_humans_delegation` | a widened audience, a one-byte-wrong secret and an unknown selector are **one** `401 invalid-delegation`; the delegated audience still mints; another human's delegation is `403`, never `404`; a listing is per delegating human, not per space |
| `only_an_author_of_the_space_can_delegate_to_an_agent` | a spectator and a non-member are both `403 forbidden`, and **a refused delegation is never written** |

`cargo check -p semio-hub --all-targets`: **0 errors** (`m6b-hub-check-1.txt`). Rule 26 forbids me
running the suite; the coordinator owns it. **Needs hub rerun** — these three laws, M6's ten
`auth::agent` laws (still unverdicted since the 09:17 capture was a truncated excerpt), and the
schema-drift law, which now has to admit the new `lastUsedAtMs` key in both the `$def` and the
decoder.

### 3.1 One new column, derived, no DDL

"List with last-used" had no source: `AgentDelegationRow` carried no such field and
`hub_agent_delegation` no such column. Adding one would have needed a migration on a
`CREATE TABLE IF NOT EXISTS` table. Instead `list_agent_delegations` computes it as
`(SELECT MAX(s.issued_at) FROM hub_auth_session s WHERE s.device_instance_id = d.id AND
s.session_kind = 'agent')` — **exactly the set `revoke_agent_delegation` cascades over**, so the
number can never disagree with what revocation would kill. Zero DDL, zero new write path, and no new
erroring default in the postgres/neo4j lanes. Landed in `AgentDelegationRow`,
`AgentDelegationSummaryV1` (`lastUsedAtMs`, always serialized so `null` is a state rather than an
absent key), the `$defs`, and the TS twin. **Observed live** (§M6b.4).

## M6b.4 Live proof — **6 of M6 §8's 7 steps observed**, and two live-only defects found and fixed

M6 §8 said "nothing in §2–§6 has been observed against a running hub" and named steps 2, 4 and 7 as
the three that had never run. All three now have.

**The rig** (capture `m6b-live.txt`, probes `🐍️m6b-live-agent-probe.ts` / `🐍️m6b-live-mcp-probe.ts`):
the coordinator's binary copied out of
`⚡️cache/cargo/target-coordinator-hub/debug/os-hub` (`BIN_DONE 12:06:34`) into
`🗑️generated/m6b-live/`, its **own** data root, port **7611** (never C1c's 7501), production posture
per `🌎️hub/README.md` — `OS_HUB_MODE=production OS_HUB_BIND=127.0.0.1 OS_HUB_CREDENTIAL_SIGN_IN=true
OS_HUB_ADMIN_SUBJECTS=…`, **no launcher and no fd 3**, first user seeded by `os-hub credential set`
with the password on stdin. Startup line:
`bind scope loopback (127.0.0.1:7611), cross-origin policy loopback-development, trusted forwarding
none`. `/readyz` is `503` on `artifactAuthority=trusted-catalog-never-published-in-this-data-root`
alone — a fresh data root has published no catalog; every auth route answers.

| M6 §8 step | observed |
|---|---|
| 1. first user → sign in | ✅ `200`, a real `session.v1.…`, `sessionKind: "external"`, a space created through `POST /directory/commands` |
| 2. `POST /auth/agent-delegations` | ✅ **`201`**, token shown once, 111-byte `delegation.v1.…`, receipt `agentPrincipalId = agent:<delegation id>` and demonstrably **not** `user:<the human>`; the listing carries the row and **not** the token |
| 3. credential file, mode `0600` | ✅ written and verified `600` |
| 4. `semio-os-mcp stdio --hub … --credential-file …` | ✅ after **defect A** below: stderr prints exactly M6 §8's predicted line, `acting as agent principal agent:01a0be4c-… ("Drafting agent") in space 01a0be4c-…`, then `real per-capability ArtifactChannel routing bound for hub http://127.0.0.1:7611/<space>`, and the JSON-RPC `initialize` answers |
| 5. `context_resolve` reports the agent principal | ✅ after **defect B** below: `"principal": "agent:01a0be4c-51c4-7365-a759-8cf5b3b3bc02"`. The hub's own view was proven separately: the exchanged session's `/auth/sessions/me` is `sessionKind: "agent"` under the delegating human's `userId` |
| 6. a directory socket roster shows presence kind `agent` | ❌ **not observed** — see below |
| 7. revoke → the MCP session is refused | ✅ `DELETE` → `204`; the **already-minted** agent session's `/auth/sessions/me` goes `200 → 401` (the cascade); a fresh `semio-os-mcp` spawn with the same credential file exits non-zero with the typed `PermissionDenied: the hub refused this agent delegation: delegation-revoked`; the withdrawn row stays visible, marked |

Two further hub behaviours observed in passing: an agent session is refused `403 forbidden` at
`POST /auth/agent-delegations` (**an agent can never delegate onward**), and the audience match is
**exact**, not "narrower is admitted" — a `read` exchange against an `edit` delegation is the same
`401 invalid-delegation` a widened one is. M6 §2.5 described only the widening case; the implemented
rule is stricter and this is the first measurement of it.

**M6b's own `lastUsedAtMs` was observed live**: `null` on the fresh listing, `1789899067847` after
one exchange. The coordinator's binary was built at 12:04:52–12:06:34, after this slice's
directory/sqlite edits at 12:03:27, so it carries them.

### 4.1 Defect A — `--hub --credential-file` panicked before it could serve anything

`🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs` `exchange_agent_session` sized its worker pool at a literal
`process_worker_pool(WorkerPoolConfig::new(InteractiveNative, 1))`. `process_worker_pool` **seals**
the process-wide configuration on its first call, and this exchange is the first thing a
`--credential-file` process does — before `NativeHubBindingDriver::connect`, before the workspace,
before the transport, all three of which read `available_parallelism()`. So the process sealed itself
at one core and the next subsystem hit a hard assertion:

```
thread 'main' panicked: process worker pool configuration mismatch: process entry point established
WorkerPoolConfig { cores: 1 }, later subsystem requested WorkerPoolConfig { cores: 10 }
```

Exit 101, immediately after printing the agent-principal line. **Every** `semio-os-mcp --hub
--credential-file` invocation on a machine with more than one core died here — M6's entire item 2 was
mechanically unusable, and no unit test could see it because the seal is process-global. Fixed at the
root: that one site now reads `available_parallelism()` like every other pool site in the crate.

### 4.2 Defect B — `context_resolve` reported the launcher's principal, not the agent's

With A fixed, the process ran and `context_resolve` answered
`"principal": "agent:local"` — the `--principal` default — while the very same process was acting as
`agent:<delegation id>` at the hub. The identity was adopted for the *credential* and nowhere else.
Fixed in `🌉️mcp/🦀️.rs` `server_for_workspace_options`: a successful exchange now rewrites the process
principal (`id` → the hub's `agent_principal_id`, `label` → the delegation label, `delegated_by` →
the principal the launcher passed) **before** the workspace is opened, so `context_resolve`, the
policy gate and the audit sink all name the same principal the hub does.

`cargo check -p semio-framework-os-mcp --all-targets`: **0 errors** (`m6b-mcp-check-1.txt`). Both
fixes were rebuilt and **re-observed**, not merely compiled (`m6b-mcp-build-2/3.txt`, and the two
RERUN blocks in `m6b-live.txt`).

### 4.3 Why step 6 was not observed

A presence row needs a **document** socket: an announced document in the space, a scoped socket
grant, and a WebSocket hello in the binary wire protocol. This hub's `artifactAuthority` gate is
closed (`trusted-catalog-never-published-in-this-data-root`), so a fresh data root cannot announce
one without first running `os-hub trusted-catalog publish`, and the MCP's own channel resolved as
`headless` rather than through a document socket. Everything step 6 exercises is unit-tested on both
sides (M6 §7.2's codec round-trip in both codecs, §7.3's two renderer laws asserting an agent and its
human appear as two rows with the badge in en and de, and the 31 shared cross-language vectors), and
the §M6b.2 label work that makes the row say `agent:<name>` rather than the human's name is
**compile-proven only**. That is strictly weaker than having seen it.

### 4.4 A UX gap this run surfaced (not fixed)

`semio-os-mcp` warns at startup that a principal launched with no `--scopes` will answer
`PERMISSION_DENIED` on every policy-gated tool. An agent credential carries an **audience** the hub
already enforces (`read` → `Spectator`, `edit` → `Author`), but nothing maps that audience onto the
MCP's own scope set, so a human who follows the delegation UI's command line exactly still gets a
scope-less agent. The UI prints the command without `--scopes` because that is the flag shape
`README.md` documents. Deriving the default scopes from the delegated audience is the obvious fix and
I did not take it: it changes the policy gate's default, which is not this slice's to widen.

## M6b.5 Files changed

**New**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🤖️delegations/🟦️.ts` — the pure delegation contract (routes, bounds, request builder, both parsers, row projection, credential-file renderer, error mapping, the legible-actor helpers)
- `…/📺️renderer/🧑‍🎨engine/🧱️elements/🤖️AgentDelegations/🟦️.tsx` — the pane
- `…/🤖️AgentDelegations/📖️stories/🧪️.story.tsx` — 10 stories
- `…/🤖️AgentDelegations/🧪️tests/🧩️component/🟦️.tsx` — 20 laws

**Modified — TypeScript**
- `…/🧱️elements/🔗️HubConnection/🟦️.tsx` — 42 `os.hub.agent.*` labels in en + de, three required port methods + `saveFile`, `AgentDelegationRefusalV1`, `HubAgentCredentialV1`, the delegation lane (state, five actions, abort/generation guards, reset on switch/forget/sign-out), the three fetch-adapter routes
- `…/🔗️HubConnection/🏛️workspace/🟦️.tsx` — mounts `🤖️AgentDelegations` with the author-only `canDelegate`
- `…/🧱️elements/🏛️ShellHost/🟦️.tsx` — `saveFile` through the existing `downloadMediaExport`
- `…/🎯️targets/⚛️react/🟦️.tsx` — exports `AgentDelegations` / `agentDelegationInstantV1` / `AgentDelegationsProps`
- `…/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` — `elementSuite("🤖️AgentDelegations", …)` in `engineTestSuites`
- `…/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts` — the new file in `hub-sign-in-spaces-check`
- `…/🧱️elements/🔐️HubSignIn/🧪️tests/🧩️component/🟦️.tsx`, `…/🏘️SpaceBrowser/🧪️tests/🧩️component/🟦️.tsx` — three port fakes widened

**Modified — hub**
- `🌎️hub/📇️directory/🦀️.rs` — `AgentDelegationRow.last_used_at_ms`
- `🌎️hub/📇️directory/🪶️sqlite/🦀️.rs` — the derived `MAX(issued_at)` sub-select and its row build
- `🌎️hub/🔐️auth/🤖️agent/🦀️.rs` — `AgentDelegationSummaryV1.last_used_at_ms`, `agent_actor_display`
- `🌎️hub/🔐️auth/🧬️schema/🔣️.json` — `lastUsedAtMs` in `AgentDelegationSummaryV1`
- `🌎️hub/🏗️bootstrap/🦀️.rs` — `SocketSubjectV1::Session.device_instance_id` (+3 construction sites), `agent_delegation_for_session`, the agent presence label, `client_label` = `agent:<label>`
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — 11 subject literals widened; **3 new integration laws**
- `🌎️hub/🔐️auth/🧪️tests/🤖️agent-unit/🦀️.rs` — two row literals widened

**Modified — os-mcp (both fixes are live-proven)**
- `🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs` — the sealed one-core worker pool (defect A)
- `🌉️mcp/🦀️.rs` — the process principal becomes the adopted agent principal (defect B)

**Ticket artefacts:** two probes (`🐍️m6b-live-agent-probe.ts`, `🐍️m6b-live-mcp-probe.ts`) and
`🗑️generated/m6b-*` (4 vitest captures, 1 renderer typecheck, 1 hub check, 1 mcp check, 3 mcp builds,
`m6b-live.txt`, `m6b-serve.txt`, and the `m6b-live/` rig).

No `📋️project.json` / `.vscode/launch.json` row was added: the delegation laws run under the existing
`hub-sign-in-spaces-check` verb and the react target's own vitest config.

## M6b.6 Honest gaps

1. **Step 6 of the live proof was not observed** (§M6b.4.3): no agent presence row has been seen on a
   real roster, because a document socket needs an announced document and this data root's
   `artifactAuthority` gate is closed. The §M6b.2 hub label work is therefore **compile-proven only**
   — it is the one M6b change with no execution behind it at all.
2. **The three new hub integration laws and M6's ten `auth::agent` laws are unverdicted.** Rule 26
   forbids me running `cargo test -p semio-hub`. `cargo check -p semio-hub --all-targets` is 0
   errors. **Needs hub rerun.** The schema-drift law in particular now has to admit `lastUsedAtMs`
   through both the `$def` and the decoder — if it is red, that is the first place to look.
3. **wgpu parity debt** (§M6b.1): no `🤖️AgentDelegations` twin under `🎯️targets/🧊️wgpu`, and still no
   `principal_kind` consumer in the native shell (M6 gap 10). A wgpu user can neither create nor
   revoke a delegation, and sees no agent badge. For WG6.
4. **An agent's MCP scopes are not derived from its delegated audience** (§M6b.4.4): the command the
   UI prints produces a scope-less agent that answers `PERMISSION_DENIED` on every policy-gated tool.
5. **`lastUsedAtMs` is sqlite-only.** Postgres and neo4j keep AU1's erroring defaults for the whole
   delegation family (M6 gap 6); the derived column adds no new gap but closes none either.
6. **History/attribution has no UI surface yet.** `actorDisplayFromRosterV1` is the function a
   history or conflict surface would call, and it is tested, but no such surface exists in the React
   shell to call it — the roster is the only live consumer today.
7. **M6's gaps 7, 8 and 9 are untouched** (the non-transactional `agent-session-issued` annotation,
   the un-rechecked `space_id` at exchange time, the unmeasured rate-limit sizing).
8. **The `ShellHost` typecheck shows one pre-existing peer error** (`consumes` on a plugin-module
   record, `🟦️.tsx:4970`) unrelated to this slice; the three errors that WERE mine (invalid
   `ControlIcon` names `bot`/`ban`, which are not in the generated icon catalog) are fixed and the
   suite re-run green.
