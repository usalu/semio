# PR1 — presence over the hub: symmetric, stable and typed

Slice PR1, ticket 26/09/18, session 7 (2026-09-21). Owns C3 §3.4 (the roster is asymmetric and
decays) and M6 §4.3 / §M6b.4 step 6 (no `agent` presence row had ever been observed).

## 0. Headline

**Both of C3's presence defects are reproduced deterministically, root-caused, and fixed; the
`agent` roster row is OBSERVED for the first time.**

Three findings, each measured on a live hub before anything was changed:

1. **The asymmetry is a join-replay gap in the hub.** A roster delta is published only when some
   peer's encoded bytes *change* (`refresh_presence`, `🌎️hub/🏗️bootstrap/🦀️.rs:2111`), and nothing
   at all is published when a socket is admitted (`install_presence_slot`, `:2068` — it returns
   `NoChange` unless it replaced a visible row). A socket that attaches after the roster has settled
   is therefore **blind to every peer already on it** until one of them moves. Measured
   (`🗑️generated/pr1-before-hub-socket.txt` run A): two seconds after the late joiner's socket
   opened it had received **0** presence frames while the first human's roster already listed a
   peer — C3 §3.4's "`user1` listed both, `user2` listed only itself", read from the hub's own side.
2. **The decay is the client's beat stopping, and the hub then erasing a live human.** The shell's
   per-document beat awaited the plugin's `ephemeralSnapshot` **unbounded** inside a `latestWins`
   single flight (`🏛️ShellHost/🟦️.tsx:8107` before the fix): a document whose component is wedged —
   exactly C3 §3.2's browser-actor state — never answers, the flight never settles, and that
   document never beats again. 15 s later the hub's `PRESENCE_LEASE_TTL_MS` lapsed and
   `expire_presence_for_live` set `slot.peer = None`, i.e. **removed a human who was still holding
   an open socket**. Reproduced with no browser at all by simply stopping the beats
   (`pr1-before-hub-socket.txt` run B): both rosters **empty** at +15 s, both sockets open,
   `sockets-never-closed-themselves: PASS`.
3. **The `agent` row was never missing — it had never been reached.** M6 landed the wire bit, the
   hub stamp and the badge; what had never existed was a delegated agent session holding a *document*
   socket. Driving that chain browser-free produced the first `principalKind: "agent"` roster row
   this product has ever shown (§5.3).

The hub half of the fix is compile- and law-verified but **not observed live**: the running hubs
execute binaries built before it (§7).

## 1. The presence path, end to end

### Hub — `🌎️hub/🏗️bootstrap/🦀️.rs`

| what | where |
|---|---|
| the per-`(document, actor)` slot: `socket_live_id`, `expires_at`, admitted identity, `principal_kind`, `peer` | `struct PresenceLeaseSlot` **:616** |
| lease length | `const PRESENCE_LEASE_TTL_MS: u64 = 15_000` **:968** |
| one actor-sorted, bounded roster from one map traversal (rows with `peer.is_some()`) | `presence_snapshot` **:2024** |
| the fan-out: `ServerFrame::Presence` to the document fanout, then the member-directory projection | `publish_presence_delta` **:2052** |
| a socket is admitted as an owner, invisible until it beats | `install_presence_slot` **:2068** |
| client beat → identity overwritten from the admitted slot, ephemerals kept | `refresh_document_presence` **:2082** (ingress at `ClientFrame::Presence` **:4792**) |
| the publication rule: **publish only when the encoded peer changed**; an identical beat only re-arms the lease | `refresh_presence` **:2111** |
| the 1 s per-socket authorization tick that evaluates the lease | `handle_ws` **:5139** |
| removal on socket close | `close_presence_for_live` **:2186**, called at **:5088** and after the loop |
| deterministic palette index: lowest free `0..=255` per space, ref-counted per actor | `acquire_color` **:2204** |

### Client — store worker and shell

| what | where |
|---|---|
| `ServerFrame::Presence` decoded, malformed rows dropped, emitted as an `ArtifactEvent` | `🏪️store/👷️worker/🟦️.ts:4158` |
| the roster is only emitted for the socket that owns the verified surface, else `peers: []` | `emitEvent` **:619**, authority set at **:4199**, type at **:519** |
| the outbound beat frame | `case "presenceHeartbeat"` **:6212** |
| per-document beat loop, 5 s | `🏛️ShellHost/🟦️.tsx:8102`, interval at **:8083** |
| roster state: the shell **replaces** its per-document roster with each snapshot — it is a faithful mirror of the hub, it never merges | `🏛️ShellHost/🟦️.tsx:3373` |
| scope filter + `principalKind → isAgent` projection | `🏛️ShellHost/👥️presence-scope/🟦️.ts:28` |
| the chrome | `PresenceBar` at `🏛️ShellHost/🟦️.tsx:11624`; row + badge + accessible name at `🖱️ui/🧱️elements/👥️PresenceBar/🟦️.tsx:143` |

**Wire.** `PresencePeer` (`📡️replication/📡️wire/🦀️.rs:1351`, TS twin `📡️replication/🟦️.ts:83`):
identity fields (`actor`, `user_id`, `label`, `role`, `color`, `surface`, `principal_kind`) are
*admitted* — the hub overwrites whatever a client sends — and the app owns `presence_pack`,
`drag_ghost_json`, `interaction`, `views`, `ui`, `tool_run`. `principal_kind` is flag bit 11.

## 2. Live reproduction, before the fix

**The browser route to a document socket is closed on this catalog and that is not this slice's
lane.** Two contexts boot, both sign in, both press Attach, `POST …/open-plan` answers **200**, and
then the chain stops: no `execution-target/*`, no `socket-grants`, no socket, and 60 s later
`Error: document opening deadline exceeded` (`🗑️generated/pr1-diag.txt`,
`pr1-diag-console-user1.txt`; the rosters stay empty for the whole 120 s,
`pr1-before.txt`). So the browser probe `🐍️pr1-presence-probe.mjs` (written, permanent, two
contexts sampling both rosters every 5 s) measured only that blocker.

**So the measurement was moved onto the hub's own sockets.** `🐍️pr1-presence-socket-probe.ts` walks
the exact chain a browser walks — `POST /auth/sessions` → `…/open-plan` → `…/socket-grants` →
`GET …/socket/v1` upgraded with `Sec-WebSocket-Protocol: semio.socket.v1, <grant>` → the
`SocketHelloV1` frame `handle_ws` waits 2 s for — with no plugin, no component and no browser, then
samples what the hub publishes to each socket. A socket's roster is the last `ServerFrame::Presence`
it received, which is exactly what the shell projects into its chrome (§1).

Two hubs, both before any change: **7611** (HS1's binary, data root `gm1-boot`, pid 48532) and
**7621** (C5's 1.34-policy hub, data root `jc1-boot`, read-only, never restarted by this slice).

| run | capture | what it shows |
|---|---|---|
| 7611 A — beats every 5 s, 120 s | `pr1-before-hub-socket.txt` | at +2 s after the late join: `user1:n=1 … frames=5 \| user2:n=0 … frames=0` — **the joiner has an open socket and zero presence frames**. Steady state afterwards is symmetric: 23/24 samples `[2,2]`, colours `[0,1]` distinct throughout, removal after close within one 5 s sample |
| 7611 B — beats stop after the first, 60 s | same file | `[1,0] → [1,1] → [0,0]` and **`[0,0]` for the remaining eight samples**, both sockets open, `sockets-never-closed-themselves: PASS`. C3 §3.4's "empty in both contexts although both sockets were still open", deterministically |
| 7621 A — beats every 5 s | `pr1-before-7621-socket.txt` | same shape on an independent hub, catalog and binary: first sample `[0,0]` (the 17 s HTTP pause while the second human's own attach chain ran was enough for the first human's lease to lapse), then `[2,2]` |
| 7621 B — beats stop | same file | `[0,0]` from +15 s, both sockets open |

The two defects are therefore **not** specific to a hub, a data root or a binary.

## 3. Roots

**(a) Asymmetry — no join replay, and deltas are change-gated.** `refresh_presence`
(`🦀️.rs:2111`) computes `changed = slot.peer.as_ref() != Some(&peer)` and publishes only when
`changed`; a steady peer re-beating the same bytes re-arms its lease and publishes nothing.
`install_presence_slot` (`:2068`) publishes only if it *replaced* a visible row, and `handle_ws`
subscribed to the fanout with no snapshot. A `broadcast::Sender` reaches only current receivers, so
the joiner's first view of the roster came from its **own** first beat. The asymmetry becomes
durable, rather than a one-round-trip window, as soon as an existing peer's lease has lapsed (root
b) — which is the ordering that produced C3's "user1 listed both, user2 listed only itself" and its
reversal in another run.

**(b) Decay — an unbounded `ephemeralSnapshot` inside a single flight, and an eviction that erased a
live human.** `latestWins` (`🔨️modules/⏳️async/🥇️latest-wins/🟦️.ts:12`) keeps `current` non-null
until the run settles; a promise that never settles means **no further beat is ever launched** for
that document. `beatOneDocument` awaited `entry.plugin.ephemeralSnapshot?.(…)` bare, and that call
crosses into the plugin — in the browser through the nested browser-actor child, the component C3
§3.2 found wedged. With the beat gone the hub's 1 s tick reached `now >= slot.expires_at` and
`expire_presence_for_live` set `slot.peer = None`: the human vanished from every roster while their
socket stayed open. Two faults compounding — the shell can stop beating for a reason that has
nothing to do with the human, and the hub treated a lapsed *payload* lease as the human leaving.

**(c) The `agent` row.** Nothing was broken. M6 landed `principal_kind` on the wire (bit 11), the
hub stamp in `refresh_document_presence` (`:2082`, `principal_kind: Some(slot.principal_kind)`), the
`isAgent` projection (`👥️presence-scope/🟦️.ts:28`) and the badge (`👥️PresenceBar/🟦️.tsx:143`).
What had never happened is a delegated agent session **holding a document socket** — M6 §4.3 says so
explicitly. §5.3 does it.

## 4. Fixes

All in product source; no shim, no compat layer.

| # | fix | where |
|---|---|---|
| 1 | **Join replay.** A joining socket is subscribed to the fanout and handed the current roster under the *same* publication gate every delta holds, so the replay can never be older than the first delta that socket receives. One frame, only when the roster is non-empty; it is the single non-delta roster frame on the wire and every other frame stays an event. | `subscribe_with_presence_replay` `🌎️hub/🏗️bootstrap/🦀️.rs:2061`, used at **:5078** |
| 2 | **A live socket is always a row.** `expire_presence_for_live` no longer hides a peer: it rebuilds the row as the identity the socket was *admitted* with, drops only the app-owned ephemerals, and re-arms the lease so a lapse publishes exactly once. Only `close_presence_for_live` ever removes a row. | `PresenceIdentityV1` **:639**, `expire_presence_for_live` **:2159** |
| 3 | **The beat is never hostage to the document component.** `presenceEphemeralSnapshotWithinBoundV1` bounds the snapshot at 2 000 ms and treats a rejection the same as a timeout; past the bound the peer beats with no pack — present, ephemerals dropped, which is what the hub publishes for a lapsed lease anyway. | `🛠️ShellHelpers/🟦️.tsx:609` (constant **:600**), called at `🏛️ShellHost/🟦️.tsx:8120` |
| 4 | **The contract is stated as constants, in one file, for both languages.** `heartbeatIntervalMs 5000`, `ephemeralSnapshotDeadlineMs 2000`, `leaseTtlMs 15000`, plus the four laws in prose. | `🧰️framework/🔨️modules/📡️replication/🧫️fixtures/💓️presence-liveness-v1/🔣️.json` (new) |

**Not changed, deliberately:** colours (`acquire_color` **:2204** already mints the lowest free index
per space and ref-counts per actor — measured distinct `[0,1]` and `[0,1,2]` in every full-roster
sample of every run), and the client's roster reducer (`🏛️ShellHost/🟦️.tsx:3373` already replaces
rather than merges, so it is a faithful mirror and needs no repair).

## 5. Measured

### 5.1 Hub laws — `cargo check -p semio-hub --all-targets` ✅

`🗑️generated/pr1-hub-check.txt`: `Finished dev profile … in 5m 19s`, exit 0, warnings only
(58 from the `os-hub` bin test target — the test target really compiled). Per rule 26 this slice ran
**no** `cargo test`/`build` on `semio-hub`.

Three laws in `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`:

- **:5121** `presence_join_replays_the_settled_roster_and_close_removes_exactly_one_row` — new. A
  settled socket beats; a second human's socket attaches and beats **nothing**, and its first frame
  after `Session` is the settled roster (right actor, right pack, right surface). Then the joiner
  beats and both sockets hold the byte-identical two-row roster with two distinct palette indices.
  Then one closes and exactly one row disappears.
- **:5064** `presence_lease_expires_server_clocked_visibility_without_socket_close` — rewritten to
  the new contract: at TTL−1 nothing, at TTL the row **survives** with `presence_pack`,
  `drag_ghost_json`, `interaction`, `views`, `ui`, `tool_run` all dropped and
  `actor`/`color` intact, published exactly once, and a second lapsed tick republishes nothing.
- **:5190** `presence_liveness_contract_matches_the_shared_fixture` — new. Reads the shared fixture
  by relative path and pins `leaseTtlMs == PRESENCE_LEASE_TTL_MS`, `snapshotDeadline < heartbeat`,
  `ttl >= 3 × heartbeat`, "only a socket close removes a row", "one replay frame per join".

### 5.2 TypeScript laws ✅ — actually run

`bun ./📜️script.ts scoped-presence-check` in `📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript`
→ **10 passed / 0 failed**, plus `scoped-presence-oracle: checks=29 clean`
(`🗑️generated/pr1-scoped-presence-vitest.txt`; M6 left it at 9). The tenth is
`📺️renderer/🧑‍🎨engine/🧪️tests/👥️scoped-presence/🟦️.tsx`, reading the **same** fixture file as the
hub law: it pins the two shell constants against it and then proves the bound on four shapes — a
document component that answers never, one that throws `browser actor child: invocation rejected`, a
healthy one (pack preserved), and a plugin with no `ephemeralSnapshot` at all.

### 5.3 The `agent` roster row — OBSERVED, first time ✅

`🗑️generated/pr1-7621-agent.txt`, live on C5's 1.34 hub, three concurrent document sockets on one
document: two humans plus a delegated agent session minted the way `semio-os-mcp --hub` mints one
(`POST /auth/agent-delegations` → `POST /auth/agent-sessions`).

```
DELEGATION id=01a0c324-fd6f-768e-ab3f-5718de6f5143 principal=agent:01a0c324-fd6f-768e-ab3f-5718de6f5143
SAMPLE user1:n=3 colors=[0,1,2] kinds=[human,human,agent] | user2:n=3 colors=[0,1,2] kinds=[human,human,agent]
     | agent:n=3 colors=[0,1,2] kinds=[human,human,agent]
VERDICT a-delegated-agent-is-its-own-roster-row: PASS — 8/9 samples carry a peer whose admitted
        principalKind is "agent" (agent:01a0c324-…); … 
VERDICT distinct-colour-per-session: PASS — 8 full-roster samples; distinct=true; colours [[0,1,2]…]
VERDICT no-decay-while-the-socket-is-open: PASS — last sample sizes [3,3,3]
```

The agent is its **own** row, never folded into the human who delegated to it, with its own palette
index — **M6 §M6b.4 step 6, which M6 recorded as "not observed"**. The label + glyph half of the row
is the vitest law of §5.2 (`data-presence-kind="agent"`, the badge, the accessible name in en and
de), landed by M6 and re-run green here.

## 6. Honest gaps

- **The hub half of the fix is NOT observed live.** Rule 26 forbids this slice from building the
  `semio-hub` binary, and every running hub executes a binary compiled before these changes (7611 =
  `⚡️cache/hs1/os-hub-7611`, 23:43 on 09-20; 7621 = `⚡️cache/cargo/target-jc1/debug/os-hub`, 03:06 on
  09-21). Fixes 1 and 2 are therefore **compile-proven and law-proven only**. To observe them,
  rebuild `.🧬semio/🦑️repo/⚡️cache/cargo/target-coordinator-hub/debug/os-hub` (or `target-jc1`),
  restart a hub on that binary, and re-run `🐍️pr1-presence-socket-probe.ts` — the "after" bar is
  the probe's own verdicts: `symmetric-roster-for-the-whole-window` PASS including the **first**
  sample (today 0/24 and 11/12 because of that first sample), and `no-decay-while-the-socket-is-open`
  PASS with `PR1_BEAT_MS=0` (today FAIL).
- **Fix 3 (the bounded beat) is not observed in a browser either**, because no browser on this
  catalog reaches a document socket at all (§2). It is proven by the vitest law on the real function,
  against the real failure shapes.
- **The browser probe `🐍️pr1-presence-probe.mjs` measured a blocker, not presence.** Its captures
  are kept as the witness that the shell's open chain stops after a 200 `open-plan`; that is JC1/C5's
  lane and this slice did not touch it.
- **Roster bounds at install.** `install_presence_slot` still admits a socket regardless of roster
  size; `presence_snapshot` clamps the published frame to `PRESENCE_ROSTER_MAXIMUM_ITEMS` and
  `…_BYTES`, so a 65th socket is simply not shown rather than refused. Unchanged by this slice and
  stated rather than silently relied on.
- **A row before the first beat.** A socket that has never beaten is still invisible; only a peer
  that has announced itself once keeps its row for the life of the socket. The shell beats
  immediately on opening a document, so the window is one round trip, and the join replay covers the
  joiner's side of it. Making the admitted row visible at install was considered and **not** taken:
  it rewrites `presence_normalization_matches_neutral_authority_and_no_effect_rejections`,
  `presence_normalization_socket_overwrites_identity_and_rejects_without_refresh`,
  `presence_lease_reconnect_rejects_old_live_refresh_and_close` and
  `presence_lease_enforces_shared_roster_bounds_and_actor_order`, and this slice may not run the hub
  suite to prove such a rewrite.
- **`👥️presence-lease-v1`'s vectors still encode the old expiry.** `🌎️hub/🧫️fixtures/👥️presence-lease-v1/🔣️.json`
  describes `tick` outcomes with `final count 0`. It is read today only for its `limits` block
  (`presence_lease_enforces_shared_roster_bounds_and_actor_order`), which is unchanged and still
  passes; its operation vectors have no executing oracle in this crate. Left alone rather than
  half-rewritten.
- **No live edit crossed.** This slice measured presence, not replication; C3 §3.2 is untouched.

## 7. State at hand-off

| what | pid / path | note |
|---|---|---|
| hub 7611, HS1's binary, data root `gm1-boot` | **48532** | started by this slice, healthy, **pre-fix binary** |
| hub 7621, C5's 1.34 hub | — | read-only, never restarted here |
| serves 6194 / 6195 | — | started at 03:40, dead after the 04:10 outage; **not** restarted, because the browser route to a document socket is blocked upstream (§2) and the measurement does not need them |

**Coordinator: please rebuild `semio-hub` and re-run the hub suite** — this slice changed
`🌎️hub/🏗️bootstrap/🦀️.rs` and `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` and may run neither.

## 8. Files changed

| file | what |
|---|---|
| `🌎️hub/🏗️bootstrap/🦀️.rs` | `PresenceIdentityV1` (:639), `subscribe_with_presence_replay` (:2061), join replay in `handle_ws` (:5078), `expire_presence_for_live` strips instead of hiding (:2159) |
| `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` | new join-replay law (:5121), new shared-fixture law (:5190), rewritten expiry law (:5064) |
| `🧰️framework/🔨️modules/📡️replication/🧫️fixtures/💓️presence-liveness-v1/🔣️.json` | **new** — the cross-language beat/eviction contract |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | `PRESENCE_EPHEMERAL_SNAPSHOT_DEADLINE_MS` (:600), `presenceEphemeralSnapshotWithinBoundV1` (:609) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | the beat uses the bounded snapshot (:8120) + its import |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/👥️scoped-presence/🟦️.tsx` | the TS twin law |

Ticket folder, new and permanent: `🐍️pr1-presence-socket-probe.ts` (the browser-free N-socket
presence probe, `PR1_BEAT_MS=0` reproduces the decay, `PR1_WITH_AGENT=1` adds a delegated agent),
`🐍️pr1-presence-probe.mjs` (the two-browser-context roster sampler), `🐍️pr1-hub-hold.ts` (a hub hold
that polls `/readyz` for 5 min instead of inheriting the shared 30 s deadline — a cold GM1 boot under
fleet load needs longer, and the shared helper's throw tears the child down with it).

Captures: `🗑️generated/pr1-before-hub-socket.txt`, `pr1-before-7621-socket.txt`, `pr1-7621-agent.txt`,
`pr1-hub-check.txt`, `pr1-scoped-presence-vitest.txt`, `pr1-before.txt`, `pr1-diag*.txt`,
`pr1-attach-diagnose.txt`, `pr1-hub-7611.txt`, and the `*-presence-socket-probe.json` /
`*-presence-probe.json` series.
