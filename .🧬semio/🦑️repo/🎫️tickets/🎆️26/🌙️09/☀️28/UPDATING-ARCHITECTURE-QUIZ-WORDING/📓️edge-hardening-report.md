# 📓️ Edge hardening report

Agent "edge" — public-edge hardening of the framework server (`🧰️framework/🛍️products/🖥️server`) and of the proctor's
edge (`🎓️teaching/🛂️proctor`: `🧩️instance`, `🎚️config`, the receipt and commit part of `🗄️storage`), 2026-10-02.
Source of the findings: `📓️audit-deploy-security.md` (B1, B2, B3, S11, N1); design §8, §9a, §13–§17.

## 1. Outcome

| Finding | Outcome |
|---|---|
| **B1** idempotency receipts leak learner ids | **Fixed.** Receipts are keyed per principal and answered only for the same target; proven through the real gateway. |
| **B2** (edge part) malformed targets activate actors; 2 MiB bodies | **Fixed.** Pre-placement admission hook, revision-0 actors are never kept, bounded LRU directory, body cap 16 KiB (largest real command: 2952 bytes), body taken at the edge within 10 s. |
| **B3** no flood protection | **Fixed.** Per-address token buckets, socket and in-flight caps, WebSocket message limits, only the routes the proctor uses, read-your-writes without a global lock, SQLite off the async workers, snapshots, fixed public error bodies, no CORS credentials — and what the load test added: group commit, bounded presence, bodies of refused requests read. |
| **Capacity proof** | **Passes** as a registered gate (`bun nx run @teaching/proctor:capacity`): 300 learners from one address, zero errors and refusals, p99 of every command and query below 80 ms alone and below 300 ms beside an abusive script that is served its allowance and no more; peak memory 108 MiB. |

Not done, by decision (section 8): a cap on plain TCP connections, per-address fairness of the presence budget,
re-checking the hub crate.

## 2. B1 — idempotency receipts

**Root cause.** `CommandBus::submit` looked a command's idempotency key up before anything else and answered the stored
receipt — whose `actor` is the learner the original command addressed — to whoever sent the key. Keys of the
enrollment saga are derived from committed events, so an anonymous caller could replay one and read a learner id, or
send it first and occupy it so that the real relay was answered as a duplicate (B2.1).

**Fix** (`🔨️modules/🎭️authority/🦀️.rs`). The stored key is `receipt_key(principal, key)` =
`"<len>:<principal key>:<key>"`: every principal has a key space of its own, the saga's service account included. A
stored receipt is answered only when `receipt.actor == envelope.target`; the same key for another target is rejected
`invalid` / `idempotency-key-conflict` and answers nothing of the stored receipt. Inside one batch of turns the same
rule holds between staged turns. The store keeps the actor's `tenant`, `kind`, `id` columns, so `erase` by actor works.

**Tests.** `authority::tests` (principal-scoped receipts, conflict, in-batch replays); end to end through the real
gateway: `an_anonymous_caller_can_neither_read_nor_occupy_an_enrollment_key` (`🧪️tests/🌐️end-to-end/🦀️.rs`) replays the
real enrollment key of a named sign-up as an anonymous caller three ways — no answer contains the learner id — and, on
a fresh proctor, sends the key *before* the sign-up: the sign-up is still relayed.

## 3. B2 — admission, placement, bodies

**Root cause.** A command reached the directory before anything judged its target: any string became an activation
(memory, a lease row) and stayed. Bodies were limited by axum's 2 MiB default.

**Fix.**
- `ServerModule::command_admission(&CommandEnvelope) -> Result<(), Rejection>` is asked by the bus first — before the
  receipt lookup, policy, placement. The proctor's is the domain agent's `Admission::admit` (ids, key shapes, target =
  the command's own actor, caps).
- An activation whose first command emitted nothing (revision 0) is passivated at once; the directory keeps at most
  2048 actors (LRU) and rebuilds an evicted one from its snapshot (every 64 events, framed with
  `Decider::state_format`, committed in the turn's own transaction) and the events after it.
- Body limit `Limits::body_bytes` = 16384 bytes, sized from a measurement (`edge_measure_command_size.ts`, kept in the
  ticket folder): the largest legitimate command is 2952 bytes. 16 KiB is 5.5 times that and 128 times below 2 MiB.
- The body is **taken at the edge** (`throttle_middleware` / `body_taken`): whole, within the limit, within
  `Limits::body_patience` (10 s), before the request counts in flight — see 5.8.

**Tests.** `a_command_for_an_id_no_actor_can_have_is_refused_before_it_costs_anything`,
`a_body_past_the_limit_is_refused_and_the_largest_real_command_is_far_below_it` (asserts the largest answer of both
catalogs times four still fits), `a_refused_request_keeps_its_connection_and_a_body_that_trickles_holds_no_place`
(end to end, raw TCP); unit: admission before placement, passivation, LRU, snapshots, `a_body_is_taken_whole_…`.

## 4. B3 — flood protection

| Part | What it is now | Where / test |
|---|---|---|
| Rates | Token buckets per client address and class (commands, reads, upgrades), integer arithmetic, `429` + `Retry-After` + `retryAfterMs`. Address = last `X-Forwarded-For` entry in proxy mode, peer otherwise, IPv6 per /64. | `🔨️modules/🚦️throttle`; oracle test against the `governor` crate (GCRA) over the fixture `🧫️fixtures/🚦️throttle/🔣️.json`; e2e `one_address_past_its_allowance_is_told_to_wait_and_no_other_is_slowed` |
| Caps | Sockets per address (2048) and in total (8192), requests in flight (2048, `503`); address table bounded (16384, idle sweep, shared seat beyond). | throttle unit tests; e2e `sockets_are_counted_per_address_…` |
| WebSocket limits | `max_message_size`/`max_frame_size`/read buffer at the protocol layer: presence 4096 bytes, event stream 1024. | e2e as above |
| Routes | `RouteGroups::CORE` only (no blobs, apps, ephemeral); `InstanceDisclosure::Public` (`GET /instance` without policy templates). | e2e `the_proctor_answers_on_its_own_routes_only_…`; wire fixture route groups, Rust and TS twin |
| Read-your-writes | Split into *relay* (after a command: the sagas' follow-ups have run) and *settle* (before a query: relay + fold), both single-flight, both skipped when the command lane's counters say nothing was committed since the last complete run. | `🧩️instance`; unit `a_request_waits_only_for_what_was_committed_…`, `callers_arriving_together_…`; e2e `every_sign_up_reads_itself_back_while_others_sign_up` |
| No global lock on reads | Store and read models are `Arc<RwLock<_>>`: event replays and queries never take the bus; queries read side by side. | `history_is_read_while_the_bus_is_held_by_a_turn` |
| SQLite | Every statement runs under `block_in_place` on the serving runtime (`off_worker`). | — (capacity gate) |
| Errors (S11) | Internal details go to stderr; public bodies are fixed (`forbidden`, `internal error`, `unavailable`). | gateway `a_fault_of_the_server_and_a_denial_of_its_policy_tell_the_caller_nothing`; wire fixture error table (now ten kinds, with `stalled` 408) |
| CORS (N1) | No `Access-Control-Allow-Credentials`; the TS client sends `credentials: "omit"`; `Retry-After` exposed. | instance and e2e tests |

## 5. What the load test revealed, in the order it was found

Every item was measured before it was changed; the numbers are from the logs of the runs (deleted with
`🗑️generated/edge/`, quoted here).

1. **The first measurements measured the load generator.** One Bun thread played 300 learners and parsed 1.2 GiB of
   presence frames, and the memory sampler called `tasklist` synchronously every two seconds. Hall-alone `record-answer`
   read p50 598 / p95 1258 / p99 2024 ms. With the sampler made asynchronous and the hall dealt to eight threads, the
   *same binary* read p50 4 / p95 16 / p99 27 ms. The gate now reports how late its own timers fire per phase; a
   latency over the budget in a phase where they fired more than 100 ms late (p99) is reported as "this machine was
   too busy to measure the hall" instead of blaming the proctor (lateness can only make the hall look slower, so a hall
   inside its budget passes), and a phase where they fired more than a second late measures nothing.
2. **Two flushes per command.** Receipt and events were separate transactions. Now a batch of queued commands is one
   transaction (`CommandBus::submit_batch`, `AuthorityStore::commit(&[TurnCommit])`, the gateway's command lane, 64 per
   batch): one flush per batch, a lone command waits for nobody. Law in the storage conformance suite:
   `a_committed_batch_lands_every_turn_whole_and_a_refused_turn_not_at_all` (reference backends and SQLite).
3. **A read-your-writes bug.** The first settle-skip compared the log head with the fold position and could skip a
   settle whose outbox rows were not relayed (3 × `unknown-learner` in 14000 requests). Fixed by recording only what a
   *complete* relay and fold covered; regression test `every_sign_up_reads_itself_back_while_others_sign_up`.
4. **Queries were serialized.** `post_query` held an exclusive lock on the read models for the whole query, behind
   every fold. Under the script the query handler took p50 10 ms. Read models behind a read-write lock: p50 0.45 ms.
5. **A command waited for the fold.** After a command the proctor relayed *and* folded before answering
   (p50 23–36 ms under the script). A command now waits for the relay only; the fold happens before the next query.
6. **Derived rows were flushed like facts.** Read models, their checkpoints and outbox delivery marks commit with
   `synchronous=NORMAL` (`Database::remakeable`) and ride along with the next flush of facts; facts stay `FULL`. After a
   power loss the proctor folds and relays again what was not on the disk (the lost commits are always a suffix).
   Test: `only_what_is_made_again_commits_without_waiting_for_the_disk`. Also an outbox cursor, so that looking for
   pending rows does not scan what is delivered.
7. **Presence was unbounded** — the largest finding. A room broadcast every batch to every member, a joiner got the
   whole roster in one frame, a lagging socket got it again, and a watcher parsed every batch of every watched room.
   With two thousand sockets of one address in one room the proctor reached 426–495 MiB; with states of the maximum
   size the same attack is tens of gigabytes. Now (`PresenceRooms`, `RoomReader`, no wire change):
   - nothing is queued for anybody: every socket reads its rooms through a cursor of a few numbers and is sent the
     newest state of what changed since its last read;
   - a frame carries at most `max_frame_bytes` (4096): a larger roster continues in the following frames, a larger
     burst over the next ticks, every changed session once before any twice;
   - a page costs what it carries, not what the room holds: seats with a tournament (max-tree) over their change
     clocks. (The first version walked the room: two cores for 2300 members. Property test against a walk:
     `the_tournament_finds_the_next_changed_seat_like_a_walk_of_the_seats_does`.)
   - all rooms together send at most `max_bytes_per_second` (16 MiB); a joining session's `welcome` is sent at once
     and counted;
   - a socket that takes no frame for 60 s is closed.
   Peak memory beside the script: 102–108 MiB. Presence bytes to the hall alone: 1375 → 560 MiB per run. Joining
   (until the `welcome`) beside the script: p99 63 ms; before the `welcome` was exempted from the total it was 3.4–4 s.
8. **A refusal closed the connection.** A `429` sent while the body was still on its way makes hyper close the
   connection. Behind Caddy on the same host those are Caddy's connections to the proctor: a flood of refused requests
   would exhaust the ports Caddy reaches the proctor through, for every client. The edge now reads a refused request's
   body (bounded, 500 ms). In the same place: an admitted request's body is taken *before* the request counts in flight,
   so 2048 trickling bodies no longer occupy every place requests are served in (`408 stalled` after 10 s).
9. **Small things.** `TCP_NODELAY` on every accepted connection (`undelayed`); colour leasing no longer quadratic.
10. **The load generator again.** Bun's `fetch` opens a new connection for every few requests once ~100 run at once
    (measured: 4800 requests, 1219 connections), which emptied this machine's client ports (16384, 120 s `TIME_WAIT`)
    and made the hall's own connections fail. The script now floods over raw sockets it keeps open (`Wire`) — which is
    also how a real script and a proxy's upstream pool behave. The hall's requests go through the quiz client's own
    retry (`retryTransient`), and the gate reports how many were sent twice (0 in the passing runs; more than 1 % fails).

## 6. Capacity proof

`bun nx run @teaching/proctor:capacity` — release build, production mode, throw-away port and data directory,
architecture catalog; hall of 300 from `198.51.100.10` at five times the pace of a lecture (enrol, roster and home
sockets with a 16-room watch at 4 Hz, leaderboard poll, two quizzes with pointer moves and thinking drafts, every
answer revised two to four times, submit, results, one Wi-Fi drop of every socket); then the same hall beside a script
from `203.0.113.66` (96 flooding connections for commands, malformed commands and queries, oversized bodies, 2560
socket attempts into the hall's own roster room, oversized frames). Budget: p95 250 / p99 500 ms alone, 500 / 1500 ms
beside the script, no error, no refusal, 512 MiB.

**The final run** (final tree, `bun nx run @teaching/proctor:capacity -- --report …`, exit 0, "the hall saw no error and
no refusal and stayed inside its latency budget alone and beside the script; the script was throttled, capped and cut
off"). Milliseconds:

| Operation | n | alone p50 | p95 | p99 | max | n | beside the script p50 | p95 | p99 | max |
|---|---|---|---|---|---|---|---|---|---|---|
| identify-learner | 300 | 2.9 | 7.7 | 19.9 | 46 | 300 | 3.1 | 15.2 | 18.5 | 22 |
| start-run | 600 | 1.9 | 8.5 | 19.9 | 42 | 600 | 2.9 | 35.2 | 75.7 | 334 |
| record-answer | 4504 | 1.9 | 10.9 | 27.2 | 170 | 4503 | 3.3 | 32.4 | 169.5 | 401 |
| submit-run | 600 | 3.6 | 10.5 | 65.5 | 335 | 600 | 5.7 | 33.5 | 174.5 | 595 |
| query catalog | 300 | 2.5 | 7.9 | 24.9 | 60 | 300 | 2.1 | 7.1 | 18.2 | 32 |
| query learner | 900 | 2.3 | 17.1 | 45.1 | 157 | 900 | 4.8 | 35.4 | 165.9 | 392 |
| query run | 1200 | 4.6 | 22.3 | 51.6 | 367 | 1200 | 7.4 | 45.0 | 220.8 | 395 |
| query crowd | 600 | 4.0 | 20.7 | 46.3 | 150 | 600 | 8.4 | 59.4 | 289.7 | 359 |
| query leaderboard | 5578 | 3.5 | 14.8 | 42.4 | 169 | 5546 | 6.2 | 38.8 | 159.7 | 508 |
| socket join (until `welcome`) | 3146 | 1.0 | 19.3 | 155.6 | 516 | 3163 | 1.6 | 10.0 | 62.8 | 771 |

| | alone | beside the script |
|---|---|---|
| Duration | 52.1 s | 52.2 s |
| Errors / refusals (`429`, `503`) / requests sent twice | 0 / 0 / 0 of 14582 | 0 / 0 / 0 of 14549 |
| Presence frames received by the hall | 204800 frames, 557 MiB | 78191 frames, 236 MiB |
| Proctor processor seconds | 11.8 (0.23 cores) | 50.0 (0.96 cores) |
| Load generator late (p99) | 11 ms | 34 ms |

The script (53.8 s): 16763 commands served (its allowance over that time: 17040) and 162038 refused `429`; 34045
queries served (allowance 34080) and 325272 refused; oversized bodies 29 × `413`, 20 × `429`, 568 cut off while still
being sent; 1982 of 2560 socket attempts opened (upgrade allowance and the cap of 2048 per address); 175 of 175
oversized frames closed their socket. The proctor: 9 MiB idle, **108 MiB at the peak**, 60 MiB at the end, healthy.

**Across runs.** Four quiet runs passed during this work: alone p95 ≤ 28 ms and p99 ≤ 78 ms, beside the script p95 ≤
59 ms and p99 ≤ 290 ms, memory ≤ 108 MiB. Two runs on a machine at 95–100 % load from other builds kept every latency
inside the budget as well (alone p99 ≤ 242 ms, beside p99 ≤ 885 ms).

**Before** (same hall; the first two columns are not comparable in absolute terms, see 5.1):

| | first run | generator fixed, batch commit in | final |
|---|---|---|---|
| record-answer alone p50 / p95 / p99 | 598 / 1258 / 2024 | 6.7 / 202 / 326 | 1.9 / 10.9 / 27.2 |
| record-answer beside p50 / p95 / p99 | 1383 / 3564 / 4837 | 85 / 227 / 325 | 3.3 / 32.4 / 169.5 |
| query leaderboard beside p50 / p99 | 1743 / 5057 | 88 / 474 | 6.2 / 159.7 |
| script refused `429` | never | yes | yes, and served ≤ allowance (checked) |
| errors of the hall | 0 (3 in another run: 5.3) | 1 (port exhaustion: 5.10) | 0 |
| peak memory | 495 MiB | 426 MiB | 108 MiB |

Server-side (temporary timing, removed again; quiet machine, hall alone / beside the script): command handler p50
1.1 / 1.3 ms (the commit itself 0.9 ms), relay 0.24 / 0.35 ms, fold 0.85 / 0.6 ms, query settle wait 0.67 / 0.46 ms,
query handler 0.34 / 0.44 ms.

What a class of 300 needs, from the same runs: 23 commands and 33 reads per second at a lecture's pace (117 and 167 at
the gate's), 850 socket upgrades within two seconds at a network drop, 900 sockets, 11 MiB per second of presence.

**Limits of the proof.** One machine carries the proctor, the hall and the script; on a machine busy with other work a
latency over the budget is reported as "not measured" (seen while other agents built, before the final runs). Latency
is loopback latency. The script is one address; several addresses each get the allowance (section 8).

## 7. Settings

| Variable | Default | Range | Meaning |
|---|---|---|---|
| `PROCTOR_LIMIT_BODY_BYTES` | 16384 | 1024…1048576 | largest request body (`413`); it must arrive within 10 s (`408`) |
| `PROCTOR_LIMIT_COMMANDS_PER_SECOND` / `…_BURST` | 300 / 900 | 1…1000000 | per address: `POST /commands` |
| `PROCTOR_LIMIT_QUERIES_PER_SECOND` / `…_BURST` | 600 / 1800 | 1…1000000 | per address: reads (`POST /queries`, every `GET`) |
| `PROCTOR_LIMIT_UPGRADES_PER_SECOND` / `…_BURST` | 100 / 1800 | 1…1000000 | per address: WebSocket upgrades |
| `PROCTOR_LIMIT_SOCKETS_PER_ADDRESS` | 2048 | 1…1000000 | sockets one address holds |
| `PROCTOR_LIMIT_SOCKETS` | 8192 | 1…1000000 | sockets in total |
| `PROCTOR_LIMIT_IN_FLIGHT` | 2048 | 1…1000000 | requests served at once |
| `PROCTOR_LIMIT_PRESENCE_FRAME_BYTES` | 4096 | 4096…65536 | what one presence frame carries |
| `PROCTOR_LIMIT_PRESENCE_BYTES_PER_SECOND` | 16777216 | 65536…4294967296 | what all presence rooms send |
| `PROCTOR_PRESENCE_TICK_MS` (existing) | 100 | 10…1000 | how often a socket is sent its room's changes |

Fixed in code: body patience 10 s, refused-body patience 500 ms, directory 2048 actors, snapshot every 64 events,
departures remembered per room 256, command batch 64, address table 16384 / idle 120 s. Documented in the proctor
README ("Environment", "Limits at the edge", "Presence") and in `🗒️edge-notes-for-deploy.md`.

## 8. Open issues

1. **TCP connections are not capped.** The proctor caps sockets and requests in flight; an idle keep-alive connection
   costs kilobytes and nothing bounds their number. Behind Caddy this is Caddy's to bound (its timeouts are set); the
   proctor's port must stay unreachable from anything else. A cap inside the proctor needs its own accept loop.
2. **Presence under a socket flood.** A script may put its 2048 sockets into the hall's own rooms (presence is open to
   every caller of the site origin). The rooms' total budget then bounds what the proctor sends, and the hall's
   presence slows down with it: the rest of a roster and the others' cursors arrive late, for as long as the flood's
   rosters are being sent (about 25 s in the gate). Commands and queries are unaffected. Fair shares of the budget per
   address, or presence only for registered learners, would be the next step.
3. **Several addresses.** Every address gets the allowance of a lecture hall. Beside one script at its full allowance
   the proctor used about half a core; a dozen addresses at full allowance would saturate the one SQLite writer. A
   global write budget would throttle the class with them; the real answer is the proxy's own limits or a registration
   allowance (next item).
4. **`roster-full` (domain).** `PROCTOR_MAX_LEARNERS` (10000) is filled by one script inside its command allowance in
   34 seconds; after that every real learner is refused until an operator acts. The gate runs with the cap at a
   million. Written to the domain agent in `🗒️edge-needs-from-domain.md`.
5. **Hub not re-checked.** `semio-hub` depends on the server crate and was not compiled (its check runs out of memory
   on this host). By reading: it uses none of the changed surfaces (`ServerState::projections`, `PresenceRooms`,
   `Limits`, `PresenceSettings`, `ServerError` matching); new trait methods have defaults (`AuthorityStore::commit`,
   `Decider::state_format`, `ServerModule::command_admission`).
6. **Clippy on dependencies.** `cargo clippy -p semio-framework-server -- -D warnings` without `--no-deps` fails in
   `semio-framework-trace`, `-dispatch-macros` and the value derive (not touched here); with `--no-deps` both crates
   are clean.
7. **Design §15** still describes one batch per room and tick and a whole roster in the `welcome`; the contract docs
   (Rust and TypeScript), the README and the tests describe the bounded frames. The wire schema did not change.
8. **A dev-e2e file touched.** `🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts`: `buildProctor(…, profile)` gained the
   `"release"` profile for the gate.

## 9. Gates (all run on the final tree)

| Command | Result |
|---|---|
| `bun nx run-many -t test -p @semio-tech/framework-server @semio-tech/framework-server-rs @teaching/proctor --skip-nx-cache` | exit 0 — TypeScript twin: 18 passed; server crate: 150 unit, 5 closed-ports, 5 wire passed; proctor: 79 unit, 15 conformance, 15 end-to-end passed |
| `cargo clippy -p semio-framework-server --no-deps --all-targets --features conformance -- -D warnings` | exit 0 |
| `cargo clippy -p teaching-proctor --no-deps --all-targets -- -D warnings` | exit 0 |
| `bun ./📜️script.ts verify taxonomy report --scope "🎓️teaching"` | `clean=true errors=0 warnings=0` |
| `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/🖥️server"` | `clean=true errors=0 warnings=0` |
| `cargo check -p teaching-proctor --all-targets` | finished |
| `bun nx run @teaching/proctor:capacity` | exit 0 (section 6) |

## 10. Files

**Created**
- `🧰️framework/🛍️products/🖥️server/🔨️modules/🚦️throttle/🦀️.rs`, `…/🚦️throttle/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/🖥️server/🧫️fixtures/🚦️throttle/🔣️.json`
- `🎓️teaching/🛂️proctor/🧪️tests/🏋️capacity/🟦️.ts`
- ticket: `📓️edge-hardening-report.md`, `🗒️edge-notes-for-deploy.md`, `🗒️edge-needs-from-domain.md`,
  `edge_measure_command_size.ts`

**Updated — framework server**
- `🦀️.rs` (`report`), `📦️packages/🦀️rust/🦀️.rs`, `📦️packages/🦀️rust/Cargo.toml` (dev-dependencies `governor`, `tower`),
  `🔨️modules/🔣️.json`
- `🔨️modules/🧬️contract/🦀️.rs` (`ServerInstanceDefinition::public`, presence frame docs)
- `🔨️modules/🎭️authority/🦀️.rs` (+ tests): admission hook, receipts, batch turns, directory, snapshots, `Committed`
- `🔨️modules/🗄️storage/🦀️.rs` (+ tests): `TurnCommit`, `AuthorityStore::commit`
- `🔨️modules/📡️gateway/🦀️.rs` (+ tests): errors, edge, command lane, route groups, disclosure, presence rooms
- `🧪️tests/🔬️conformance/🦀️.rs`, `🧪️tests/🔒️closed-ports/🦀️.rs`, `🧪️tests/🧩️instance/🦀️.rs`, `🧪️tests/🔬️wire/🦀️.rs`,
  `🧪️tests/🔬️wire/🟦️.ts`, `🧫️fixtures/🔌️wire/🔣️.json`, `🟦️.ts` (TS twin)

**Updated — proctor**
- `🔨️modules/🎚️config/🦀️.rs` (+ tests), `🔨️modules/🧩️instance/🦀️.rs` (+ tests), `🔨️modules/🗄️storage/🦀️.rs` (+ tests)
- `🧪️tests/🌐️end-to-end/🦀️.rs` (region `🔖️Edge`), `🧪️tests/🔬️conformance/🦀️.rs`
- `README.md`, `📦️packages/🦀️rust/📜️script.ts`, `📦️packages/🦀️rust/📋️project.json`, `🏗️bootstrap/🟦️.ts`

**Updated — elsewhere**
- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (row `⚖️gate🎓️teaching🛂️proctor🏋️capacity`)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (`🚦️throttle` in modules and fixtures, `🏋️capacity`
  in tests)
- `Cargo.lock`

**Removed**
- `🗑️generated/edge/` in the ticket folder (logs, reports, scratch binaries). No source file was removed.
