# Warden Victim Choice and Retry After a Memory Cancel

## Evidence
- `repo run @semio-tech/gis-js:test --lang=en` failed with `build:nx-command failed (1)` (log `🗑️generated/ts-target-gis-js.log`).
- The failure was `HostWardenCancelledErrorV1`: "cargo pid 26488 (bun ./script.ts generate, 216 MB): the host was critically short of memory. Nothing is retried."
- Free memory was 68 % shortly after; long gismap cargo test compiles held 1.5-1.8 GB grants at the time.

## Cause
1. Victim choice (`decideHostWardenV1`, `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🟦️.ts`): the danger cancel sorted its candidates with `order` (suspended first, then the least wall time per megabyte). A young small step has the lowest value, so the 216 MB cargo step went before the 1.5-1.8 GB processes, freed too little and was then followed by more cancels (up to 2 per tick). `cargo` and `stage` are cancel-only kinds (cannot be suspended); they become candidates once every pausable subject is already suspended past `pauseDeadlineSeconds`, which was the case.
2. Recovery: `launchRepositoryCargoV1` called `requireNoHostWardenCancelBelowV1` after a failed cargo run and threw the typed error; the message said nothing is retried and nothing re-queued the build.

## Fix
- Suspension still comes first (a pausable subject is a cancel candidate only after it stayed suspended past the pause deadline; a cancel-only subject only when every pausable one did). A subject over its cap or ceiling is still cancelled first. The remaining candidates now go in the order `dearer`: not a declared final link, then the largest, then suspended before running, then the old cheapest-to-lose order. A small step is never cancelled before a large process. The Python oracle (`🧪️tests/🐍️.py`) mirrors the new order.
- Retry: `retryAfterHostWardenCancelV1` (`🔁️loop/🟦️.ts`) runs the work; on a `HostWardenCancelledErrorV1` whose cancels are for memory (`hostWardenCancelRetryableV1`: not a cap or ceiling cancel) it reports a bilingual line, waits for a calm host (polls the warden state every `requeuePollMilliseconds`, reports progress each poll, cancellable through the abort signal, at most `requeueCalmMaximumSeconds`; an absent warden counts as calm) and runs again, so the work passes capacity admission again. At most `requeueMaximumAttempts` (3) attempts; afterwards the typed failure carries `retries` and says it was queued again N times. Cap and ceiling cancels, and all other errors, are not retried ("Nothing is retried" stays for them).
- New policy numbers and bilingual messages in `🧬️schema/🔣️.json` (`requeue*`, `Messages.*.requeue`, `failure.exhausted`).
- Wiring: `🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🚪️launcher/🟦️.ts` wraps `admitted(...)` in `retryAfterHostWardenCancelV1`; `launchedMs` is now per attempt, so the cancel check of a later attempt ignores the earlier cancel record.

## Tests (run via `repo test-file ... --lang=en`)
- Warden suite `🛡️warden/🧪️tests/🟦️.ts`: 57 pass. New: victim choice (small young cargo step versus a large suspended process, oracle fixture row `victim: a severe danger ...`, and nothing is cancelled while suspension can still recover); requeue after a warden cancel (waits for the held state, reports both languages, admitted again); bounds (3 attempts then the typed failure with retries, cap cancel not retried, other errors pass through, wait cancellable).
- Host-capacity suite `🏗️build/🌡️host-capacity/🧪️tests/🟦️.ts`: 20 pass. New: a build cancelled by the warden releases its grant and is admitted again through `withBuildCapacityV1` (two admissions, no grant left).
- Fixture `🧪️tests/🧫️fixtures/🔣️.json` got one decision row, expectation written by the Python oracle (`--write`, `--check` ok).

## Not verified
- The launcher wiring is syntax-checked only (bun build); no cargo run and no end-to-end rerun of the gis-js test.
- `cargo` stays a cancel-only kind (a test asserts that); making it suspendable was not changed.

# Addendum: Slice Workers Killed Inside Their Grant

## Evidence (warden records of 2026-11-26, `~/Library/Caches/semio/host-capacity/warden/`)
- 15 cancel records, 13 of them `slice-worker`, 12 with reason `danger` (the 14th is a `cap` at 6528 MB over 6144 MB): 17:56 16297 (3645 MB), 17:57 26358 (2669), 18:00 32821 (2310), 18:02 43456 (2859), 18:03 54044 (4173), 18:07 59414 (2833), 18:10 89865 (4343), 18:23 38674 (4215), 18:28 3958 (2243), 18:36 13590 (2541), 18:47 72808 (2672), 18:52 79299 (3561).
- 72808 (2672 MB, inside the 2762 MB grant of the planner report): `resume calm` 18:46:09, 18:46:44, 18:47:13, then `suspend danger` 18:47:21, `cancel danger` 18:47:40. 79299: `resume calm` 18:50:35 and 18:50:51, `suspend danger` 18:51:23, `resume calm` 18:51:25, `suspend danger` 18:52:28, `cancel danger` 18:52:56.
- The decision was a genuine danger: at 18:52 the host had 2827 MB available but swapped out 973 MB/s with pressure warn (the `acute` path, severe). The log named it "memory pressure is critical", which was wrong (the pressure was only warn) and said nothing about the process.
- The `resume calm` lines on pool-stopped workers: the loop sets the suspended-since time of every stopped subject when first seen (`since`), so a process the pool stopped with SIGSTOP while it waits for grant growth counted as suspended by the warden: it was resumed by a calm host while the pool was still waiting for the gate, and would have been cancelled as overdue after `suspendMaximumSeconds`.

## Cause
(a) The warden knew nothing of the grant: a worker inside its booked memory was an ordinary danger candidate, ordered only by size. (b) A stop by the owner looked like a stop by the warden. (c) The log blamed "memory pressure" for acute swapping and named neither the grant nor the reason of the pick.

## Fix
- New owner claim (schema `Claim`, `claims/<pid>.json` in the warden directory; `claimHostWardenSubjectV1`, `releaseHostWardenClaimV1`, `readHostWardenClaimsV1`; ignored when older than `claimMaximumMilliseconds`, 10 min): the grant the owner holds at the gate and whether the owner itself stopped the process. `applyWardenClaimsV1` (pure) puts it on the subject: `grantMegabytes`, and for an owner-stopped process `held` (not suspended, `canSuspend` false), so it is never overdue, never resumed and never suspended by the warden.
- (a) A subject within its grant is cancelled as overdue never unless the level is danger and severe, and in the danger cancel order it goes after every subject not within a grant, the subject furthest above its grant first (`dearer`: not sheltered, furthest above grant, not a final link, largest, suspended, cheapest). A cancel for danger still happens when the host is truly in danger, as before.
- (b) The slice pool (`🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`, new port `claims`) writes the claim at the worker start and after each growth with the booked grant, with `stopped:true` around its SIGSTOP while it waits for the gate, and releases it when the worker ends.
- (c) New trigger `swapping` (danger from acute swap-out without critical pressure, with its English and German text) replaces the false "memory pressure is critical". Alarm lines and cancel records now carry `grantMegabytes` and say "it held 2672 MB, within its grant of 2762 MB, and was taken only because the host was in danger" or "653 MB above its grant of 3247 MB" (English and German, also in the typed failure line). The Python oracle mirrors the order, the overdue rule and the trigger.

## Tests
- Warden suite: 60 pass (new: sheltered versus over-grant victim and overdue rule and swapping trigger via four oracle fixture rows plus the grant wording in both languages; claims applied/stale/round trip; MOUNTED: an owner-stopped worker is never resumed or cancelled, an unclaimed stop is resumed by the calm host).
- Host-capacity suite: 20 pass. Slice-pool grant-growth suite: 5 pass (run with `bun test` directly, because `repo test-file` currently refuses the resident suite budgets: "/files/7/reason must contain at most 240 character(s)", unrelated to this change), now also asserting the claim sequence (grant at start, stopped with the grant, running with the larger grant, released at the end).

## Not verified
- No live rerun of `dev-s-react-prototype`; the running warden restarts itself on the changed source digest, the resident service must be restarted for the pool change.

# Addendum 2: Sheltered Workers Still Cancelled, and the Gate

## (1) Was the claim present and honored?
- The warden ran the new code: the 19:47:06 and 19:54:59 cancel lines carry the new words ("swapping memory out at a high rate", "it held 3253 MB, within its grant ..."), so claim and grant were read (`grantMegabytes` 5054 and 3770 are in the alarms). So the claim was present; the order rule alone did not hold, for two reasons found in the decision:
  1. A cancel candidate must have been paused for `pauseDeadlineSeconds`. The sheltered worker (suspended first, as the largest, 19:46:56) reached that deadline while the unsheltered subjects, suspended in the same tick or later, had not, so the only eligible candidate was the sheltered one (cancelled 10 s later). The sort order never helped because the eligible set held one subject.
  2. A worker the pool stops for grant growth is `held` (`canSuspend` false). A subject that cannot be suspended is eligible as soon as every pausable subject is paused (vacuously true when none remains), so a held worker became a direct cancel candidate with no suspend first: pid 69170 at 19:54:59 (3418 MB within 3770 MB, no earlier suspend alarm, claim `stopped` written around growth).
- Fix: (a) a sheltered subject is not a cancel candidate while a subject not within a grant still waits for its pause deadline, and it is suspended after the unsheltered ones; (b) a `held` subject is never a cancel candidate (its owner's stop is not a failed suspension; the claim ends within 10 minutes). The Python oracle mirrors both. Fixture rows: "a sheltered worker paused past the deadline ..." and "a worker its owner holds stopped ..." (both expect no action). Warden suite: 60 pass.

## (2) Admission overcommit
- Live ledger at 19:55: 7 graph-plan grants plus 2 quiet runs reserve 7.4 GB in total on a 16 GB host, but the host had 2978 MB available, 5.3 GB swap in use, 7.7 GB compressed and load 10: the gate's books did not exceed capacity, the machine did. The memory the machine used beside the gate (unmanaged: the editor, other agents, compilers outside a grant) and the workers growing past their reservation between two one-second samples is where the 16 GB went.
- I fuzzed the pure decision (`decideHostCapacityV1`, shipped policy, 20 000 random hosts, ledgers and queues, with retained graph-plan reservations, settled observations, unmanaged memory and growth requests with a minimum; script `gate-overcommit-fuzz.ts` in the ticket folder): no case admitted more than total - floor - unmanaged beside the held grants, and a growth request waits with reason `memory` until its minimum is free, then takes at most the room. So there was no admission bug to fix in the gate; I did not change its law (I could not show a violating case, and a change without one would cost throughput).
- Regression tests (`🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🧮️overcommit/🟦️.ts`, 2 pass): the admitted sum never exceeds the carry for 4 000 random cases; a growth request waits while no memory is free and then takes between its minimum and the room.
- What remains open (needs a decision, not found a bug): grants that settle are charged by observation, so the room an idle worker's reservation does not use is given to others and is not there when it grows; the slice pool judges growth at 90 % of the grant once per second; memory outside the gate is only subtracted, never limited. The honest levers are a larger floor or a lower `budgetBasisPoints` in `🔣️policy.json` for a 16 GB host, or booking the plan ceiling of a slice worker as its reservation; I left the policy numbers as they are.

# Addendum 3: Slice Worker Reservation Booked Up Front

## Decision
The coordinator decided: book the slice worker's reservation up front as the larger of its plan ceiling and the last measured peak of its scope; floor and `budgetBasisPoints` stay as they are; grant growth stays as the fallback.

## Implemented
- Pool (`🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs`):
  - The booking is `max(plan ceiling share of the host, learned peak of the scope)`, bounded as before by the elasticity maximum and the host cap (`sliceBookingV1`). On a 16 GB host that is 6553 MB.
  - The first ask carries that booking as request and as minimum: the gate admits it in full or the load queues in the fair queue (progress as before), instead of starting on a reduced share (2056 MB of 6553 in the dry run).
  - If the gate refuses the full booking (never fits, unsatisfiable), the load falls back to the earlier smaller booking (counter `bookedSmaller`), and the grant growth, the owner claims and the re-admission at the measured peak keep working.
  - The peak is read and written per scope (`learned.read(scope)`, `learned.write(megabytes, scope)`, scope = the root of the load).
- Persistence (`🗓️scheduler/📈️learn/🟦️.ts`, host command `⌨️command/🌐️host/🟦️.ts`): `hostSliceLineageV1(scope)` = the host-wide class `slice:graph` plus `slice:graph/<hash of scope>` in the host estimate store (with its decay, quantile and outlier rules); a scope never measured reads the host-wide class.
- Note: because the plan ceiling (60 % of the host) is above the host cap (40 %, 8192 MB maximum), the booking is in practice the cap; the per-scope peak matters on hosts where it is larger than the guard share and for the re-admission after a cut.

## Tests
- Slice-pool grant-growth suite (8 pass, `bun test`): the first ask has minimum equal to its megabytes and equals the elasticity maximum; the peak is read and written with the scope of the load; a refused full booking falls back to the smaller booking (second ask with a smaller minimum, `bookedSmaller` 1) and the load completes.
- Learn suite (3 pass): per-scope peaks differ, an unmeasured scope reads the host-wide class.
- Adapted to the new semantics: `📐️booking` (first ask is 6553 as request and minimum) and `🛟️reservation` (host fixed at 16384 MB, wave 3146); with `✂️cut-load`, `🛡️plan-guard`, `🔁️restart`, `🚦️slice-priority`, `🤝️slice-share`: 60 pass, 0 fail across the nine files.
- Not run: the resident service must be restarted for the pool change; no live dry run (the planner lane does that). `repo test-file` is refused for the resident suites by the invalid test-file budget entry named in addendum 1, so they ran with `bun test`.

# Addendum 4: Starvation of the Up-Front Booking

## Cause
- The 6,553 MB full-or-queue request starved because the gate's only barrier (`blocked`/`reserved` after `wait.bypassMilliseconds`) applies when the request is `reservable`, which needs no soft stop; this host sits at memory pressure warn almost all the time, so the large request never became a barrier and smaller grants kept taking the 2.7 to 4.7 GB that were free.
- The per-scope peak read 5,941 MB: the store's decayed quantile with its margin kept stale larger history.

## Fix
1. Size (`⚙️`pool `🟨️.mjs`, `🗓️scheduler/📈️learn/🟦️.ts`): a scope with a measurement books its measured peak times 1.15 (`measuredBookingBasisPoints` 11500 in the resident schema), bounded by the cap, in full (request = minimum, falls back to the smaller booking when refused). A scope without a measurement asks for the plan ceiling but accepts a reduced start (minimum 2048) and grows through the existing growth path. The per-scope peak is now recency-weighted: the largest peak of the newest 5 runs of the scope (`HOST_SLICE_RECENT_RUNS_V1`, at least `minimumSamples` runs, else the host-wide class), never above the ceiling; with the five peaks of today (3.4 to 4.5 GB) after older runs of 5.8 to 6 GB it reads 4.5 GB, so the booking is 5,175 MB instead of the cap.
2. Fairness (`host-capacity/📏️decision/🟦️.ts`, `wait.headOfLineMilliseconds` optional in the schema, 60 000 in the shipped `🔣️policy.json`): the oldest request that asked for memory and waited at least that long is the head of the line, whatever the memory pressure says. While it waits, a newer request is admitted only if it fits beside the memory the head needs (its cost plus the head's need within the free memory and the free budget); otherwise it waits with reason `queue-order`. Grow requests of held grants, quiet, elastic, lent and rebuild requests are not the head and not blocked by it. The head itself is admitted by the ordinary rules as soon as the holders release.

## Tests
- Gate (`🧮️overcommit/🟦️.ts`, 3 pass): before the bound a small request passes a waiting 6,553 MB request; after it two small requests wait with `queue-order` while the head waits for memory; with the holders released the head is admitted with its full 6,553 MB.
- Learn suite (4 pass): per-scope peaks, host-wide fallback, and the recency case (four stale 5.8 to 6 GB runs, then 3.4 to 4.5 GB: 4,500; never above the ceiling).
- Slice pool: grant-growth suite (8 pass: measured peak times 1.15 as request and minimum, an unmeasured scope asks for the cap with a smaller minimum, per-scope read and write, fallback on refusal); with booking, reservation, cut-load, plan-guard, restart, slice-priority and slice-share 58 pass, 0 fail (`bun test`). Host-capacity suites: framework 29, shipped-policy 1, products 20 pass.
- Not run: live dry run (the planner lane); the resident service must be restarted for the pool change.

# Addendum 5: Reduced Start for a Booking the Host Can Never Give

## Cause
Run 18's 5,143 MB full booking sat at the head of the line for 26+ minutes with one active grant and about 2.9 GB available: roughly 5 GB of the 16 GB are held outside the gate (fseventsd 2.7 GB, the Claude apps), so a full booking of that size can never be met, and full-or-queue has no way out.

## Fix
- Request fields (schema `Request`, `HostRequestV1`, optional): `floorMegabytes` (the reduced start accepted, below `minimumMegabytes`), `queuedSinceMs` (when the operation first queued) and `expectedMilliseconds`. `reserveGraphPlanCapacityV1` and the host command pass them through.
- Gate (`📏️decision/🟦️.ts`): a request with a floor is relaxed once it waited at least `max(wait.headOfLineMilliseconds (60 s), expectedMilliseconds)` since `queuedSinceMs` (else its arrival), or once the most the host could ever give (every holder released: `drainedRoom`/`drainedBudget`) is below its booking. A relaxed request is admitted with what is free when that is at least its floor (reason `shrunk`), also under a soft stop such as pressure warn, and it counts with its floor as the memory the head of the line needs, so newer requests are no longer held back for memory it cannot get.
- Head-of-line priority for growth: the age of a request is `queuedSinceMs`, so a growth request of a load that queued early is the head of the line over requests that queued later; the pool puts the queue time of the first booking on its grant and on every growth ask.
- Pool: the measured full booking asks with `floorMegabytes` = the elasticity minimum (2,048 MB) and `queuedSinceMs`; grant growth, the owner claims and the warden's within-grant and pool-held protection carry the reduced start.

## Tests
- `🧮️overcommit/🟦️.ts` (4 pass): a 5,143 MB booking with a 2,048 MB floor and about 2.9 GB free is admitted reduced (at least the floor, less than the booking) once it waited past the bound; an operation expected to run ten times the bound keeps waiting; a booking the host could never give is reduced at once; and the growth of an old booking (queued 60 s ago) keeps head-of-line priority over a request that queued later, which waits.
- Regression: framework host-capacity 29, gate 34, reserve 2, lend 7, products host-capacity 20 pass; slice-pool suites 58 pass (`bun test`).
- Not run live; the resident service must be restarted for the pool change.
