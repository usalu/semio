# Presence Convergence Under Chaos (2026-11-19)

Why the dev-topology chaos run could end with a device counting 98 or 99 of 100 learners online, and what was fixed.

## Symptom

- The run ends with every quiz submitted, the proctor's records correct and every connection calm.
- The final stage "presence converges on everyone online" then polls for 90 s and one or two learners stay missing.
- Run 3 of the repeated dev runs is the first with the new roster audit. It names the cause: **the proctor's own roster room misses `Chaos 8`**.
  - It holds 110 sessions of 99 learners.
  - The sessions without state are the audit's own and the bots' duplicate sockets.
  - It is therefore not a device that lost sight of a learner, but a learner whose state the proctor does not hold.

## Root Causes

### 1. Timers Judged Healthy Sockets on a Starved Page (Client)

- On a starved or throttled page, a timer can run after the frames that arrived meanwhile, which are still queued behind it.
  - The join timer (10 s) and the silence watchdog (25 s) then closed live sockets.
  - Starved pages kept flapping, rejoining and being closed again.
- Fix in `🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🔨️modules/👥️presence/🔌️socket/🟦️.ts`:
  - `stallAwareTimer` notices when it ran more than `PRESENCE_STALL_MS` (1 s) late.
  - In that case it gives the socket `PRESENCE_STALL_GRACE_MS` (one heartbeat) more instead of closing it.
- Tests in `🧰️framework/🛍️products/📚️course/🧪️tests/🚪️presence-room/🟦️.tsx`:
  - a frozen page's late watchdog keeps the socket
  - a frozen page's late join timer keeps the socket
  - a truly dead socket is still closed after the grace period

### 2. The Proctor Dropped the Newest State Over the Rate (Server)

- `admit_frame` counted every client frame against `max_states_per_second` (30 per one-second window, counted on arrival).
  - It **dropped the excess silently**.
- Presence is a "latest state wins" stream, and the client sends each state once.
  - After an outage, a frozen page or proxy buffering, frames arrive in a burst.
  - A dropped frame can be the newest one, and nothing ever resends it.
  - The session then stays joined with an old state, or with none, until the page happens to publish again.
  - At the end of a run nobody publishes any more, so the learner stays missing for good.
- Fix in `🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/👥️presence/🦀️.rs` and `🛰️presence-stream/🦀️.rs`:
  - `admit_frame` answers `Admitted::Now` or `Admitted::Held`.
  - The session keeps the newest held `state` and the newest held `watch` (`Held`).
  - `StateBudget::refills_at` tells when the allowance is whole again; the held frames are applied then.
  - A frame of the same kind applied in the meantime supersedes the held one.
  - The rate is still enforced (a burst is slowed down), but its last word is never lost.
- Tests:
  - `a_state_frame_is_bounded_by_size_rate_and_shape` (unit): frames over the rate are `Held`, not dropped.
  - `a_burst_of_states_over_the_rate_still_ends_in_the_newest_state` (session law): with a rate of 3/s, a burst of 10 states ends with the room holding the 10th, after the next window.

## Diagnostics Added to the Chaos Test

- `rosterAudit` (protocol learner) reports:
  - the sessions and learners in the proctor's roster room
  - which expected learners are missing
  - sessions without state
  - states a client would reject (`presenceProblem`)
- A failing convergence check now also describes every missing browser learner. That includes the roster socket's trail: when it opened, whether and when it was welcomed, how many states it sent and when the last one, and anything refused.

## Results

- With only the client stall fix, the dev topology passed 2 of 3 runs. The failing run's audit showed the proctor's roster lacking one learner.
- With both fixes:
  - dev topology: 3 of 3 runs pass (16.3, 20.0 and 20.4 min), presence 100 every time
  - static release with the release proctor: passes (17.2 min), with 1000 runs each scored once
- Unit and companion suites:
  - presence room (client): 19/19
  - course-react: 1153
  - presence crate: 18/18
  - gateway companion: 121/121
  - proctor workspace: 133/133
- Deployment: the client fix is in the staged CDN folder. The server fix ships with the next proctor build, so the proctor must be redeployed.
