# R14: the fleet wait is bound by absence of progress

Landed 2026-11-15 05:52 CET (04:52Z) under `T/landing.lock`, recorded in `T/landings.md`. 10 files in `🧰️framework/…/🏗️build/🪪️graph-state/🙋️wanted/` (schema, `🟦️.ts`, `📖️read`, `🛠️server`, `💰️cost`, and five existing test files). No new non-test file. Script: `T/🧰️scripts/R14/land.py` (three-way merge, parse proof, tests; `files.json`, `base/`, `new/`).

## Causes found in the code

1. The reader's bound was "no record arrived for 900 s". A server busy in one wave (or behind another requester) delivers nothing for longer, though it heartbeats, so the reader refused (exit 75) while the server was working.
2. The request lived 20 min (`ttlMilliseconds`) and was never renewed. The server dropped the held request at expiry and stopped steering, so a longer wait was impossible anyway.
3. The server never renewed its claim on a held request, so after 20 min a second serving machine could claim the same request (duplicate serving).
4. A re-ask got a new `createdMs`, so a new claim key, a new "serving" line and a lost position.
5. The priority file ordered heavy first, then cost, with no regard to how many requests ask for a project. Already ok: the refresh read is steered by held requests, and a reader already infers the rest itself when `missing <= localProjects` (`settled`).

## What changed

- Progress record (schema `Served`, optional fields): `total`, `progressAtMs` (a read started, or the dirty set of the read changed, i.e. a wave published), `remainingMilliseconds` (null if nothing measured). Remaining time comes from what this read finished for the request (observed rate), else from the measured cost table (seconds per owner, median for unmeasured, `waveProjects` parallel).
- Reader (`awaitWantedV1`): progress is an arrival or a change of the server's `progressAtMs`. It gives up only after `waitMilliseconds` (now 30 min, was 15) without progress (cause `stalled`), after `silenceMilliseconds` (5 min) without any sign of life (cause `silent`, nobody holds the request), or at `waitMaximumMilliseconds` (now 6 h, safety net, cause `limit`). The refusal stays the typed `unserved`, exit 75; the cause is printed as a line before it (en/de). The refusal's own value set is unchanged because the text catalog rejects extra placeholders.
- Wait line: `graph: waiting for the fleet: x of N projects arrived; the server last progressed s s ago, estimated time left: about m min` (en/de). The age is computed from the server's own clock difference plus local elapsed time, so clock skew does not matter.
- The reader renews its request every `renewMilliseconds` (5 min) with the same `createdMs`, the missing projects only and a later expiry. Same claim key, so the serving machine keeps its claim, its running read and its position.
- Server: renews the claim of every held request every 5 min (a lost claim is left to the other machine, with a line); reads the renewals through a new `live` callback of `claimWantedV1`; a new request of the same requester joins the held entry (keeps what is fresh and the position, prints `joined`, no second `serving` line); the progress record is published on every change and with each heartbeat; priority order is now demand (number of held requests asking for the project) first, then heavy, then cost, then age. A project two workers wait for is inferred once and first.

## Proof (all in the existing test files, overlay run with `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`)

Red on the old code, green on the new (checked by running the new tests against the baseline implementation: 2 read tests, 3 server tests and the two import-level suites failed; "what arrived is used" passed on both, because that property already held):

- `📖️read`: "Law: the read waits as long as the server's progress record advances" (progress every 60 s, nothing delivered for 38 min, then 560 of 600 arrive; reader waits 38 min, request still alive at 35 min with the same `createdMs`, wait lines in en and de); "no progress for the bound is a typed refusal even though the server's sign of life never stops" (refuses at 1800 s, cause `stalled`, `silent` after 300 s, both languages, exit 75); "what arrived is used" (70 missing, capacity 72: returns `served` at once).
- `🛠️server`: "two requests overlapping by 400 projects are served by one read" (priority 800 unique names, the 400 shared first, one read, both answered with `total`, `remainingMilliseconds` 0); "a request is idempotent" (outlives its first expiry by renewals, claim renewed at least twice, re-ask joins: one `serving`, one `joined`, one read, final answer only the remainder); "the progress record is honest" (progress stands still while only heartbeats go out, moves when the dirty set shrinks; estimate 12 000 ms from the table before any project finished).
- `🧪️tests` (main): `awaitWantedV1` law for moving (40 min), standing (`stalled`), silent, endless (`limit`) servers with renewals; request renewal and claim-key identity; Ajv validation of the `Served` schema with and without progress, damaged progress refused, en/de texts have the same placeholders.
- `💰️cost`: demand ordering and the remaining-time estimate (table, median, observed, null, zero).
- Updated by intent: the priority test now expects `b` (asked by two requests) first.
- After landing, 18 test files green: the five wanted suites plus two-platforms, idle-grant, wave-rows, priority, cost (2), the four sidecar graph tests, head-rank, too-heavy, three fleet-read suites.
- Dry run before landing was clean (all ten files equal to the baseline, all parse).

## Real log after landing (10 minutes, `/Users/Shared/semio/.cache/f/sidecars/build-agent.log`)

Real lines, UTC: `04:59:36 refreshing the fleet generation in the background`, `04:59:41 serving 540 stale projects for g1-m04`, `05:00:12 serving 632 stale projects for g1-m07`; then 3 min 23 s without a restart and without a new line; `05:03:35`/`05:04:06 agent code changed, restarting` followed again by `serving 632 …`/`serving 540 …`. No `joined`, `answering` or `lost` line appeared in the window, and no worker retry was visible there.

## Not verified, and what others must know

- Not verified live: the new reader against a real group server on a real 16 GB worker. The window showed no completed served answer, so no real "x of N" wait line or progress record was observed. No `repo` build command was run (rule: no graph commands on this machine).
- Major finding, not mine to fix: during the whole watch the build agent restarted with `agent code changed, restarting` every 30 to 60 s (04:52:28, 04:52:59, 04:55:00, 04:55:31, 04:56:02, 04:57:03, 04:57:34, 04:59:34, 05:03:35, 05:04:06), caused by landings of the parallel agents. Every restart drops the held requests and the running read of the in-process graph server; "serving 540 … for g1-m04" printed again after each one. As long as that churn lasts no server can finish a long read, whatever the reader does. The reader's renewal (same `createdMs`) makes the restarted server re-claim and serve it again, but the read itself restarts from the published waves. Recommendation: run the graph server out of the agent's restart closure (own process, restarted only when the server module changes), or debounce restarts while a held request is being served.
- `waitMilliseconds` 1800 s equals about the server's own `inferStallMilliseconds` (1500 s) plus margin: a server whose read stalls restarts its read, which counts as progress.
- Old servers (processes not yet restarted on the new code) publish no progress fields: the reader then falls back to the arrival-or-heartbeat rule with the 30 min bound, still longer than before.
- `N/LAND-OK` did not exist; I wrote it for the landing and removed it afterwards (as permitted). The landing lock was held by R10 for about 5 min first; I waited for it.
- Backup of the originals was deleted after the green test run; rollback would be a three-way merge of `T/🧰️scripts/R14/base/` over the live files.
