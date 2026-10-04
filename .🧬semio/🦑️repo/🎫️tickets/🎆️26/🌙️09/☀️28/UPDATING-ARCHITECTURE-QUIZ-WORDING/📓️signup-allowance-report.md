# 📓️ Sign-up allowance, registration cap, pruning and presence shares

Agent "sign-up", 2026-10-02. Scope: the framework server (`🧰️framework/🛍️products/🖥️server`), the proctor
(`🎓️teaching/🛂️proctor`), the default cap in the quiz core (`🧰️framework/🛍️products/❓️quiz`), the capacity gate and the
runbooks. Problem: edge report §8.4 (one script fills `PROCTOR_MAX_LEARNERS` in 34 s and every learner after it is
refused `roster-full`), §8.2 (presence under a socket flood), §8.3 (several addresses). Every command and number
below was run on this host on the final tree unless it says otherwise.

## 1. Outcome

| # | Item | Outcome |
|---|---|---|
| 1 | Sign-up allowance per client address | **Done.** A named allowance class in the framework (`Limits::allowances`, `ServerModule::command_allowance`), the proctor classes every `identify-learner` under `sign-up`: 1200 at once, 100 per hour, `PROCTOR_LIMIT_SIGNUPS_BURST` / `…_PER_HOUR`. Only accepted registrations count. |
| 2 | Default cap | **Done.** Measured 1.7 to 6.5 KiB of disk and no memory per registration; default cap 10000 → **100000** (170 to 650 MiB at the cap); one address needs **41 days** to reach it. |
| 3 | Operator cleanup | **Done.** `proctor prune --older-than <age> [--dry-run]`, nx target `@teaching/proctor:prune`, two launch rows in both launch files, runbooks in both READMEs. |
| 4 | Capacity gate | **Done.** Runs with the default cap; the script tries to fill the roster while the hall signs up; passes (two runs, section 6). |
| 5 | Presence fairness | **Implemented.** The rooms' budget is dealt max-min fairly among client addresses; unit-tested, and the gate now fails if the hall keeps less than half of its presence beside the flood (59 to 61 % measured, 42 % before). One flooding address still halves the budget of a hall that asks for more than half (section 5.5, open issue 2). |
| 6 | Client presentation | **Checked, wire detail added, UI not changed.** A `429` of the sign-up allowance carries `"allowance":"sign-up"`; today's client retries it silently under a "server busy" notice. What the UI should do is in `🗒️signup-notes-for-ui.md`. `roster-full` is presented sensibly already. |

Not done: no edit in the React target (by instruction); the hub crate was not compiled (open issue 5).

## 2. The sign-up allowance (item 1)

### 2.1 Design

- **Framework, domain-neutral** (`🔨️modules/🚦️throttle`, `🔨️modules/📡️gateway`):
  - `Limits::allowances: Vec<Allowance { name, rate }>` — named token buckets per client address beside the three
    request classes. `Throttle::spend(client, name, now)` / `Throttle::refund(client, name, now)`.
  - `ServerModule::command_allowance(&CommandEnvelope) -> Option<&'static str>` — the instance classes a posted
    command; the default classes none. `post_command` spends the named token **after** the request's own write token
    and before the turn, and hands it back when the outcome kept nothing (`gateway::kept`: a rejection, or an accepted
    command without events, i.e. a replay). The token is never held by the request: a caller that hangs up while its
    command commits has paid. Commands the instance issues itself (the enrollment saga) never pass `post_command`.
  - `Rate` is now `{ tokens, per, burst }` (`Rate::per_second`, `Rate::per_hour`): one bucket implementation, exact in
    integers for any period. The fixture `🧫️fixtures/🚦️throttle/🔣️.json` is v2 and gained an hours-scale schedule; the
    oracle test still runs every schedule through the `governor` crate.
  - **An address is forgotten only when it is what a fresh one would be**: idle for `Limits::idle`, no socket, and
    every bucket full again. Before, an address idle for two minutes was forgotten — fine for per-second rates, but it
    would have handed a script a fresh sign-up allowance every two minutes (36000 registrations per hour).
  - **An IPv6 client is its /48, no longer its /64.** A subscriber is commonly routed a /56 or /48 (a tunnel broker
    hands out a /48 for free): keyed by /64, one home connection had 256 to 65536 addresses, each with every allowance.
    This applies to all classes, not only sign-ups.
  - `ServerError::Throttled { wait, allowance }`, `ErrorBody.allowance` (omitted when absent), also in the TypeScript
    twin (`ErrorBody.allowance`, `ServerCallError.allowance`, `decodeErrorBody`, `encodeErrorBody`) and in the wire
    fixture (`error-body-names-the-allowance-that-is-spent`).
- **Proctor** (`🔨️modules/🎚️config`, `🔨️modules/🧩️instance`): `SIGN_UP = "sign-up"`, `SIGN_UPS = Rate::per_hour(100, 1200)`,
  `PROCTOR_LIMIT_SIGNUPS_PER_HOUR`, `PROCTOR_LIMIT_SIGNUPS_BURST` (1…1000000);
  `command_allowance` = `envelope.kind == "quiz.identify-learner"` (anonymous and handle claims alike; the admission
  holds a command of that kind to a payload of that kind). The startup line names the allowance.

### 2.2 Sizing and arithmetic

- **Honest use.** A hall of 300 signs up within a minute: 300 tokens. The next hall right after, and everybody who
  cleared the browser signing up again: 900. The burst of **1200** is the hall four times over, and only accepted
  registrations count — a handle that is taken (`handle-claimed`) or refused, `learner-exists`, a malformed command and
  a replay cost nothing —, so mistyping never eats it. **100 per hour** hands a hall's worth back in three hours and
  the whole burst in twelve: the same hall a week later meets a full allowance. Test
  `the_sign_up_allowance_never_refuses_a_lecture_hall_and_keeps_one_address_from_the_cap_for_weeks` plays eight weeks
  of 300 + 300 (a lecture later) + 300 (again) from one address without a refusal; the gate plays two halls within two
  minutes.
- **Abuse.** One address registers at most 1200 + 100 per hour. Time to fill a cap C: (C − 1200) / 100 hours.

  | Cap | Before (300 commands/s, no allowance) | With the allowance |
  |---|---|---|
  | 10000 (old default) | 34 s | 88 h (3.7 days) |
  | **100000 (new default)** | 5.5 min | **988 h = 41 days** |

  k addresses need 41 / k days (open issue 1).
- **Wait told to a refused sign-up:** the time the next token takes, at most 3600 / 100 = 36 s.

### 2.3 Tests

- Throttle unit: fixture schedules incl. the hours-scale one against `governor`;
  `a_named_allowance_is_a_bucket_of_its_own_and_takes_back_what_changed_nothing`;
  `an_address_is_remembered_until_every_bucket_of_it_is_full_again`;
  `an_ipv6_client_is_the_whole_block_a_subscriber_is_routed`.
- Gateway unit: `a_classed_command_spends_its_named_allowance_and_is_handed_back_what_kept_nothing` (through the real
  router: rejections and replays cost nothing, the third creation is `429` with `allowance`, `Retry-After: 60`,
  another address has its own); `a_command_kept_something_when_it_committed_an_event_or_was_deferred`; the refusal
  body with `allowance` in `a_refusal_names_its_wait_…`.
- Wire: Rust and TypeScript round-trip the new vector; TS `decodeErrorBody` refuses a non-string `allowance`.
- Proctor unit: config defaults, overrides and refusals; the sizing test above;
  `a_registration_is_the_one_command_counted_against_the_sign_up_allowance`.
- Proctor end to end (real HTTP, behind the proxy gate):
  `sign_ups_are_counted_per_address_by_what_they_register_and_a_spent_allowance_is_named` — 24 refused sign-ups and
  replays cost nothing, the fourth registration of the address is `429 … "allowance":"sign-up"` with the CORS grant,
  the refused learner does not exist, a registered learner plays on, a recall by `quiz.handle` works, another address
  has its own three, and a restart counts six registrations.

## 3. The default cap (item 2)

Measured with `signup_measure_learner_cost.ts` (kept in the ticket folder) against the release build: 20000
registrations of each kind over one data directory; "live" is `proctor.sqlite` plus its write-ahead log, "compact" a
`proctor backup`:

| Registration | Live disk | Compact | Resident memory |
|---|---|---|---|
| anonymous | 1742 B | 1341 B | 12.3 → 34.6 MiB during the first 20000 (actor directory, SQLite cache), flat afterwards |
| pseudonym, 16 letters | 2893 B | 2796 B | +0 (34.6 MiB) |
| name, 64 letters of three bytes (the maximum) | 6480 B | 6300 B | +1.6 MiB over 20000 (36.1 MiB) |

A registration that never plays costs disk only: no learner is kept in memory that does not play (the board holds
ranked learners only; the actor directory is an LRU of 2048).

**Decision: `DEFAULT_LIMITS.learners` = 100000** in both cores (`🧬️schema/🟦️.ts`, `🧬️schema/🦀️.rs`, the Python
reference, the schema description, the lifecycle fixture and its generator). At the cap the disk holds 170 MiB (all
anonymous), 290 MiB (ordinary pseudonyms) or 650 MiB (the worst a client can construct); one address needs 41 days to
get there. A pruning of 100000 streams took 54 s with the debug build and left a 90 KiB file (section 4).

## 4. `proctor prune` (item 3)

```text
proctor prune --older-than <age> [--dry-run]      age: <whole number><s|m|h|d|w>, e.g. 90m, 36h, 7d, 2w
```

A verb of its own rather than an option of `erase`: `erase` names exactly one learner and refuses anything else;
`prune` removes by rule. It removes, from a stopped proctor's database (`Database::open_offline`):

1. every learner that never submitted a run and registered longer ago than the age, with every handle it holds;
2. every handle claimed that long ago beside the identity of a learner that stays (a learner is its first
   registration; the site never claims a second handle, a script can — and could otherwise shelter junk handles under
   one learner that played).

Who submitted a run stays. Removal follows the rules of `erase`: whole streams with receipts, outbox rows, deliveries,
snapshots and leases, every read model dropped, the file rewritten (`secure_delete`, `VACUUM`, WAL truncated).

- **Counts, never names**: the report (stdout) prints registrations, learners by identity kind, handles and what stays.
- **Progress and cancellation**: learners go in batches of 512, each batch one transaction and always a learner with
  all of its handles (a handle left without its learner would be relayed to it again at the next start). Stderr
  reports every ten percent; Ctrl+C / `SIGTERM` stops between batches, what was removed is scrubbed, exit `130`.
- **Nothing to prune changes nothing** (read models stay).
- **A bug found by the test and fixed**: a handle that goes while its learner stays left the receipt of its enrollment
  with that learner; the next claim of the handle is relayed under the same key to another learner, which the bus
  refuses as `idempotency-key-conflict` — the handle would have been claimable but its claimant never registered.
  `actors::enrollment_receipt` names that receipt and `Database::remove(actors, receipts)` removes it.
- Storage: `Database::remove` matches the actors as one set through a temporary table (the receipt and outbox tables
  have no index by actor; a pass per actor would be quadratic), `Database::scrub`, `Database::erase` = `erasure` +
  `remove` + `scrub`; `stream_spans` (one grouped pass: first event time, event count, submitted runs) and
  `first_events`.

Tests: `prune_takes_one_age_with_a_unit_and_at_most_one_dry_run`,
`the_stale_registrations_are_those_nobody_played_under_before_the_horizon`,
`prune_removes_what_nobody_played_under_from_a_stopped_proctor_and_every_player_stays` (refused while served, dry run
changes nothing, bytes of names, ids and handle streams gone from every file, registration count 12 → 2 after the
rebuild, standings unchanged, freed handle relayed to its next claimant),
`streams_are_seen_at_a_glance_and_removed_as_one_set`; end to end with the built binary:
`the_operator_prunes_the_registrations_nobody_played_under_and_every_player_stays` (72 registrations → 2, 91 streams,
views of the players byte-equal before and after, pruned ids and handles usable again).

Live through nx: `PROCTOR_DATA=<60000 registrations of the measurement> bun nx run @teaching/proctor:prune -- --older-than 30m --dry-run`
→ `would prune 60000 registrations … (20000 anonymous, 20000 under a pseudonym, 20000 under a name), holding 40000 handles`,
exit 0; the same with `--older-than=30m` → ten progress lines, `removed 100000 of 100000 streams: 100000 events, 100000
receipts, 100000 outbox rows …`, `done: … 0 registrations are left`, exit 0, 54 s, database 216 MiB → 90 KiB.

Registration: nx target `prune` (`📋️project.json` → `bun ./📜️script.ts prune`), `PruneScript` in `📜️script.ts`
(`proctorSelection`, the former `proctorErasure`, normalizes forwarded flags for both verbs), launch rows
`🧹️prune🎓️teaching🛂️proctor🔍️dry-run` and `🧹️prune🎓️teaching🛂️proctor` (order 213.617, 213.618) and the input
`proctorPruneAge` (default `7d`) in **both** `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`. Runbooks: proctor
README "Prune (registrations nobody played under)", site README "Pruning registrations nobody played under".

## 5. Presence fairness (item 5)

5.1 **What was wrong.** One budget for all rooms, and every socket drew from it as it asked: an address got the share
its sockets were of all sockets. Two thousand flooding sockets against the hall's nine hundred left the hall a third.

5.2 **What it is now** (`gateway::Outflow`, `Share`). Every address that reads holds two levels beside the whole: an
**equal part** filling at budget / addresses reading — a page is never refused while it is above nothing, whatever
others take — and a **fair part** filling at the rate the budget was last dealt at; past its equal part an address
reads only while its fair part and the whole are above nothing. Every 250 ms the budget is dealt max-min fairly: an
address that was refused no page is given what it took, what those leave is split evenly among the addresses that
were refused one. Because an address below an even split is never refused, what is measured of it is its real demand.
Total over any stretch: the budget, half a second's worth and a page per reader.

5.3 **Contained**: `Outflow` (about 90 lines), `RoomReader` carries the client key, `PresenceGrant.client` (set by
`admit_presence`), `PresenceRooms::read(lane, lead, client)`. No wire change. An instance without a presence budget
(`max_bytes_per_second: None`) is untouched.

5.4 **Tests**: `the_budget_is_dealt_fairly_among_addresses_however_many_readers_each_has` (alone: all it asks; beside
200 flooding readers: an even split from the first half second, half each when both want more, all it asks when it
asks for less with the rest going to the flood, restored when the flood leaves),
`what_modest_addresses_leave_is_split_evenly_among_those_that_ask_for_more` (100 small addresses keep every byte,
the two large ones halve the rest), `a_reader_is_not_held_back_by_what_another_address_took` (through
`PresenceRooms`), the two older budget tests unchanged.

5.5 **Measured in the gate**: beside 1979 to 2021 flooding sockets the hall was sent 340 of 559 MiB and 331 of 560 MiB
(61 %, 59 %), where the edge agent measured 236 of 557 MiB (42 %). The hall alone asks for up to the whole 16 MiB/s
at its busiest (10.9 MiB/s on average), so its even split of 8 MiB/s clips its peaks: one flooding address does slow
such a hall's presence, to exactly half the budget, not to a third. A hall that asks for less than half is not slowed
at all. The gate asserts at least 50 % (`PRESENCE_SHARE`).

## 6. Capacity gate (item 4)

`bun nx run @teaching/proctor:capacity` — release build, **default cap** (the gate removes `PROCTOR_MAX_LEARNERS`
from its environment; the million is gone). New in the scenario and the verdict:

- the script's command flood is the roster-fill attempt: 24 connections post registrations (half anonymous, half
  handle claims) while the hall signs up; its answers are classified (`registered`, `refused` = `429` naming
  `sign-up`, `throttled`, `rejected …`);
- registered ≤ the sign-up allowance over the time it ran, ≥ the burst (something else must not have stopped it),
  refused in the name of the allowance at least once, never `roster-full`;
- after the proctor stopped, its own `prune --older-than 0s --dry-run` counts the roster: the hall's 600 learners all
  played and hold 600 registrations, what a prune would remove equals what the script registered, and everything
  together is below 5 % of the cap;
- the hall keeps at least half of its presence bytes beside the script.

Both runs passed (exit 0). Milliseconds, run 2 (final tree):

| Operation | n | alone p50 | p95 | p99 | n | beside p50 | p95 | p99 |
|---|---|---|---|---|---|---|---|---|
| identify-learner | 300 | 2.4 | 6.4 | 10.5 | 300 | 2.8 | 8.9 | 17.4 |
| start-run | 600 | 1.8 | 6.3 | 14.3 | 600 | 2.0 | 14.8 | 66.6 |
| record-answer | 4451 | 2.1 | 10.4 | 20.7 | 4421 | 2.2 | 18.5 | 72.0 |
| submit-run | 600 | 3.9 | 9.1 | 25.6 | 600 | 4.3 | 11.0 | 26.0 |
| query catalog | 300 | 1.9 | 4.9 | 8.7 | 300 | 1.7 | 5.3 | 9.6 |
| query learner | 900 | 2.4 | 15.0 | 25.1 | 900 | 2.1 | 14.0 | 28.4 |
| query run | 1200 | 4.4 | 19.1 | 34.3 | 1200 | 4.8 | 17.7 | 43.5 |
| query crowd | 600 | 4.4 | 20.3 | 38.3 | 600 | 4.9 | 17.0 | 28.6 |
| query leaderboard | 5553 | 3.7 | 12.3 | 23.3 | 5495 | 4.8 | 18.9 | 70.0 |
| socket join | 3160 | 0.9 | 3.6 | 9.3 | 3171 | 1.4 | 6.3 | 18.5 |

| | Run 1 | Run 2 |
|---|---|---|
| Hall errors / refusals / requests sent twice | 0 / 0 / 0 in both phases | 0 / 0 / 0 in both phases |
| Worst p95 / p99 alone | 34.5 / 94.0 ms | 20.3 / 38.3 ms |
| Worst p95 / p99 beside the script | 30.7 / 171.8 ms | 18.9 / 72.0 ms |
| Script: registered / allowance / refused `sign-up` / other `429` of its sign-ups | 1201 / 1202 / 12133 / 105218 | 1201 / 1202 / 11304 / 122122 |
| Script: commands passing the command allowance / queries served | 17714 / 35943 in 57.2 s | 16643 / 33740 in 53.5 s |
| Roster at the end | 1801 of 100000 (1.8 %): 600 played, 1201 idle | the same |
| Presence kept by the hall | 61 % (340 of 559 MiB) | 59 % (331 of 560 MiB) |
| Proctor: processor seconds alone / beside; peak memory | 17.1 / 36.6 s; 108 MiB | 11.0 / 21.8 s; 109 MiB |
| Load generator late (p99) alone / beside | 28 / 20 ms | 8 / 11 ms |

## 7. Client presentation (item 6)

Read in `🎯️targets/⚛️react`: `proctorTransport` turns every `429` into `ProctorThrottled(retryAfterMs)`;
`QuizSession.identify` retries through `retryTransient` until cancelled; the form shows "working" and the header
"Server busy – retrying shortly". `roster-full` has its own sentence (`quiz.rejection.rosterFull`) and is not retried.
So a sign-up refused by the allowance is retried silently and gets in when a token has flowed in (at most 36 s for
one learner; much longer for a queue of them). The server now says which allowance is spent (`"allowance":"sign-up"`);
carrying it to the form and ending the silent retry is the UI's work, written down with wording in both languages in
`🗒️signup-notes-for-ui.md`. I changed nothing in the React target.

## 8. Gates (final tree)

| Command | Result |
|---|---|
| `bun nx run-many -t test -p @semio-tech/framework-server @semio-tech/framework-server-rs @teaching/proctor @semio-tech/quiz @semio-tech/quiz-rs @teaching/architecture-quiz --skip-nx-cache` | exit 0, `Successfully ran target test for 6 projects` — server TS 18 passed; quiz TS 278 passed (10 files); architecture-quiz 129 passed (5 files); server crate 157 unit, 5 closed-ports, 5 wire; quiz crate 98 passed (2 filtered); proctor 85 unit, 15 conformance, 17 end to end. Ran four times: twice exit 0 (the last on the final tree), twice exit 1 with every test passed and `@semio-tech/quiz-rs` killed by its 15 s budget while cargo waited for the other projects' build locks (open issue 6). |
| `bun nx run @teaching/proctor:capacity` | exit 0, twice (section 6) |
| `cargo clippy -p semio-framework-server --no-deps --all-targets --features conformance -- -D warnings` | exit 0 |
| `cargo clippy -p teaching-proctor --no-deps --all-targets -- -D warnings` | exit 0 |
| `cargo clippy -p semio-framework-quiz --all-targets --features sut -- -D warnings` | exit 0 (after fixing three `needless_borrow` in `✅️validation/🦀️.rs` that the task-icon work had left) |
| `cd "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" && SEMIO_TEST_BUDGET_MS=900000 RUSTC_WRAPPER="" bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | `[test] level=exhaustive cases=13 executed=105 passed=105 failed=0 errored=0 parity=105/105` |
| `bun ./📜️script.ts verify taxonomy report --scope "🎓️teaching"` | `clean=true errors=0 warnings=0` |
| `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/🖥️server"` | `clean=true errors=0 warnings=0` |
| `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"` | `clean=true errors=0 warnings=0` (`🖼️task-icons` registered in `members-of-tests`) |

## 9. Open issues

1. **Several addresses.** The allowance is per address: k addresses fill the cap in 41 / k days, 83 addresses at once.
   Nothing per address stops that; what bounds it is the cap (disk), the proxy's own limits, and `prune`. A global
   sign-up allowance was considered and rejected: five addresses could then starve every sign-up for as long as they
   keep asking, a cheaper lever than the one removed.
2. **Presence beside a flood.** One flooding address leaves a hall half of the presence budget; a hall that asks for
   more at its busiest (the gate's does) is sent less presence then — 59 to 61 % over a lecture. Doubling
   `PROCTOR_LIMIT_PRESENCE_BYTES_PER_SECOND` removes that for one flooding address at twice the bytes sent to the
   flood; k addresses leave 1 / (k + 1).
3. **A replay while the allowance is spent** is answered `429` like a new sign-up (the edge cannot know it is a replay
   before the turn). The client keeps its command id and gets the stored receipt after the wait.
4. **IPv6 /48** groups every customer of a carrier that shares a /48 into one client for all classes (300 commands
   per second, 2048 sockets, 1200 sign-ups). Right for this proctor; another instance of the framework may want the
   prefix length as a limit of its own.
5. **Hub not compiled** (its check runs out of memory on this host). By search it names none of the changed surfaces:
   no `Limits`, `Rate`, `ServerError::Throttled`, `PresenceGrant`, `PresenceRooms::read`, `ErrorBody`;
   `ServerModule::command_allowance` has a default.
6. **`@semio-tech/quiz-rs:test` and its 15 s budget.** In the six-project `run-many` cargo waits for the locks of the
   other projects' builds; twice the quiz crate's run finished all tests (98 passed) and was then killed at 15 s.
   Not caused by this work, but it makes that gate flaky on a cold or contended build.
7. **The React client** retries a sign-up refused by the allowance silently (section 7).
8. **`prune` removes a learner with an open, never submitted run** once it is older than the age — by design; choose
   the age longer than a first quiz takes.
9. **Disk per registration** is dominated by the outbox, which keeps every event's payload twice after delivery;
   trimming delivered outbox rows would about halve it. Not done.
10. **Design document** (`📓️design.md`) describes none of this; the READMEs and this report do.
11. **Docstring emojis**: every docstring I added starts with an emoji that is unique in its file; the files have
    older duplicates (`gateway`, `config`, `storage`, the gate) that I left.
12. **Edge §8.1** (plain TCP connections are not capped) is untouched.

## 10. Files

**Created**

- Ticket: `📓️signup-allowance-report.md`, `🗒️signup-notes-for-ui.md`, `signup_measure_learner_cost.ts`

**Updated — framework server** (`🧰️framework/🛍️products/🖥️server`)

- `🔨️modules/🚦️throttle/🦀️.rs`, `🔨️modules/🚦️throttle/🧪️tests/🔬️unit/🦀️.rs`, `🧫️fixtures/🚦️throttle/🔣️.json`
- `🔨️modules/📡️gateway/🦀️.rs`, `🔨️modules/📡️gateway/🧪️tests/🔬️unit/🦀️.rs`
- `🧪️tests/🧩️instance/🦀️.rs`, `🧪️tests/🔬️wire/🦀️.rs`, `🧪️tests/🔬️wire/🟦️.ts`, `🧫️fixtures/🔌️wire/🔣️.json`, `🟦️.ts`

**Updated — proctor** (`🎓️teaching/🛂️proctor`)

- `🔨️modules/🎚️config/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`, `🔨️modules/🧩️instance/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`,
  `🔨️modules/🎭️actors/🦀️.rs`, `🔨️modules/⌨️cli/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`,
  `🔨️modules/🗄️storage/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`
- `🧪️tests/🌐️end-to-end/🦀️.rs`, `🧪️tests/🏋️capacity/🟦️.ts`
- `README.md`, `📦️packages/🦀️rust/📜️script.ts`, `📦️packages/🦀️rust/📋️project.json`, `🏗️bootstrap/🟦️.ts`
  (`proctorErasure` → `proctorSelection`)

**Updated — quiz core** (`🧰️framework/🛍️products/❓️quiz`)

- `🧬️schema/🔣️.json`, `🧬️schema/🟦️.ts`, `🧬️schema/🦀️.rs`, `🧬️schema/🧪️tests/🔬️unit/🦀️.rs` (default cap 100000)
- `🧪️tests/🧾️learner-lifecycle/🐍️.py`, `🧫️fixtures/🧾️learner-lifecycle/🔣️.json` (limits and quota vectors), `README.md`
- `🔨️modules/✅️validation/🦀️.rs` (three needless borrows clippy refused)

**Updated — elsewhere**

- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (two rows, one input)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (`🖼️task-icons` in `members-of-tests`)
- `🎓️teaching/🏛️architecture/❓️quiz/README.md` (Deploy: pruning, limits), `🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/compose.yaml`
  (comment: the new variables travel through `.env` like every other `PROCTOR_*`)
- Ticket: `generate_quiz_vectors.py` (quota vectors follow `DEFAULT_LIMITS`)

**Removed**

- `🗑️generated/signup/` in the ticket folder (logs, gate reports, the measurement's data directory). No source file.
