# 🗒️ Sign-up allowance: what the UI has to know

Written by the agent "sign-up" (server and proctor crates) on 2026-10-02 for whoever owns the React target
(`🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react`). I made **no edit in the React target**; nothing there needed one to
compile. Details and numbers: `📓️signup-allowance-report.md`.

## 1. What changed on the wire

| | Before | Now |
|---|---|---|
| A sign-up (`identify-learner`, anonymous or claiming a handle) from an address that registered too much | never refused at the edge; the roster filled and everybody got `roster-full` | `429` with a body that **names the allowance** (below) |
| `ErrorBody` (`@semio-tech/framework-server`) | `{ kind, message, retryAfterMs? }` | `{ kind, message, retryAfterMs?, allowance? }` — also `ServerCallError.allowance` |
| Default cap of registrations (`DEFAULT_LIMITS.learners`, `PROCTOR_MAX_LEARNERS`) | 10000 | 100000 |
| `roster-full` | a rejection (`200`, `status: "rejected"`, detail `roster-full`) | unchanged, and now weeks away from any one address |

The refusal of a sign-up whose address has spent its sign-up allowance, exactly:

```http
HTTP/1.1 429 Too Many Requests
Retry-After: 36
Access-Control-Allow-Origin: https://quizzes.architektur-und-technologie.de
Access-Control-Expose-Headers: retry-after

{"kind":"throttled","message":"the sign-up allowance of this address is spent","retryAfterMs":35912,"allowance":"sign-up"}
```

- `allowance` is **absent** from every other `429` (the address simply sends too fast: commands, queries, socket
  upgrades). Those are worth waiting out silently, as the client does today.
- `allowance: "sign-up"` means: the request was not too fast — this network (one client address: one IPv4 address, one
  IPv6 /48) has registered as many learners as it may for now. 1200 at once, 100 more per hour; `retryAfterMs` is the
  time until the **next single** sign-up of that address is possible (at most 36 s), not until everybody waiting gets
  in.
- Only registrations that were **accepted** count. A handle that is taken (`handle-claimed`) or refused
  (`handle-invalid`), `learner-exists`, a malformed command and the replay of a sign-up under the same command id are
  handed their token back. So the form may be submitted as often as a learner mistypes.
- Nothing else is affected while the allowance is spent: who is registered plays on, and **returning to a pseudonym is
  a read** (`quiz.handle`), not a sign-up — `QuizSession.identify` already recalls before it registers.
- While the allowance is spent even the replay of an accepted sign-up is answered `429` (the edge cannot know it is a
  replay before the turn). Keep the command id and send it again after the wait, as `retryTransient` does.

## 2. What the client does with it today (read, not changed)

- `proctorTransport` (`🔨️modules/🛂️proctor/🟦️.ts`) turns **every** `429` into `ProctorThrottled(retryAfterMs)` before the
  body is looked at beyond `retryAfterMs`; `allowance` is dropped.
- `QuizSession.identify` (`🔨️modules/🧭️session/🟦️.ts`) registers through `retryTransient`, which waits `retryAfterMs` and
  sends the same command again — forever, until the learner presses *Cancel*.
- The identity form shows `quiz.identity.working` with *Cancel*; the reachability turns `throttled` and the header says
  "Server busy – retrying shortly" / "Server ausgelastet – neuer Versuch in Kürze".
- `roster-full` is shown with `quiz.rejection.rosterFull` ("The quiz server has reached its maximum number of
  learners. Please try again later, or continue with a pseudonym that already exists.") — sensible, keep it.

So a learner refused by the sign-up allowance is **not stuck and not wrong**, but not told the truth either: the form
spins under a "server busy" notice, and gets in by itself once a token has flowed in. With one or two learners waiting
that is seconds; with a whole hall waiting behind a spent allowance (which takes more than 1200 sign-ups from one
address within hours — a script inside the same network, or four full halls at once) the last one would wait hours
while the form says "retrying shortly".

## 3. What I suggest the UI does (yours to decide and build)

1. **Carry the allowance.** `ProctorThrottled` gets `allowance: string | undefined`, read from the body next to
   `retryAfterMs` (`askedWait` already parses it; `ServerCallError.allowance` exists for calls that go through
   `ServerClient` errors).
2. **Do not retry a sign-up silently when `allowance === "sign-up"`.** End `identify` with a failure the form can show
   — a third `SessionFailure` kind, e.g. `{ kind: "waiting", allowance: "sign-up", retryAfterMs }` — instead of
   looping. The learner then sees why, and presses *Sign up* again when they want to (the allowance may be free by
   then; pressing again costs nothing, a refused attempt is not counted).
3. **Say what is true and what helps**, in both languages (du-form in German), for example:
   - EN: "Too many new sign-ups from your network right now. Please try again in a minute — or continue with a
     pseudonym you already have."
   - DE: "Aus deinem Netzwerk haben sich gerade sehr viele neu angemeldet. Versuch es in einer Minute noch einmal —
     oder mach mit einem Pseudonym weiter, das du schon hast."
4. **Leave the general `429` path as it is**: silent retry and the "server busy" notice are right for it.
5. `roster-full`: nothing to change.

Server-side examples to test against: `sign_ups_are_counted_per_address_by_what_they_register_and_a_spent_allowance_is_named`
in `🎓️teaching/🛂️proctor/🧪️tests/🌐️end-to-end/🦀️.rs` (run a proctor with `PROCTOR_LIMIT_SIGNUPS_BURST=3` and
`PROCTOR_LIMIT_SIGNUPS_PER_HOUR=60` to meet the refusal after three sign-ups), the wire vector
`error-body-names-the-allowance-that-is-spent` in `🧰️framework/🛍️products/🖥️server/🧫️fixtures/🔌️wire/🔣️.json` and
`decodeErrorBody` in `🧰️framework/🛍️products/🖥️server/🟦️.ts`.

## 4. Things you may trip over

- The dev proctor has the allowance too (1200 at once per address; a loopback browser is one address). An end-to-end
  run that signs up more than 1200 learners from one address within hours needs `PROCTOR_LIMIT_SIGNUPS_BURST` raised
  in its launcher environment.
- `DEFAULT_LIMITS.learners` is 100000 in both cores; a test that counted on 10000 has to follow
  (`🧫️fixtures/🧾️learner-lifecycle/🔣️.json` and its generator are updated).
- The capacity gate (`🎓️teaching/🛂️proctor/🧪️tests/🏋️capacity/🟦️.ts`) still drives `ProctorClient` and `retryTransient`
  as they are; it needs no change of yours.
