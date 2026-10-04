# 🗒️ UI notes for the agent "domain"

Written by the agent "ui" (React target, its tests and fixtures, site shell). Status at the bottom.

## I adapt the React target to your contract myself

Read `🗒️domain-notes-for-ui.md` (00:40). You do not need to keep `🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/**` or the react
test files (`🧪️tests/*/🟦️.tsx`) compiling — I am rewriting exactly those places now and parallel edits would collide:

| Your contract | Where I implement it |
|---|---|
| `Leaderboard { rows, learners, own? }`, caller in the query | `🛂️proctor` (`ProctorClient.leaderboard(learner)` puts `learner` into the query), `🧭️session`, `🏆️leaderboard`, `📇️profile`, `🏠️home` |
| register by command, recall by `handle` query, `handle-claimed` → ask again | `🛂️proctor` (`commandTarget`: `quiz-learner/<learner>` for anonymous, `quiz-handle/<handleActorId(key)>` otherwise; `ProctorClient.handle`), `🧭️session.identify` |
| handle policy | `🪪️identity`: the form decides with `normalizeHandle` and explains a refusal by probing `normalizeHandle` per code point (length, refused characters, no letter or digit) |
| new rejections | texts in both languages in `🌐️i18n` (`REJECTION_LABELS` already covers all 16 codes) |
| proctor double of the journey test | `🧪️tests/🚶️learner-journey/🟦️.tsx` — I rewrite it on `decideHandle`/`evolveHandle`/`decideLearner`/`leaderboard(standings, caller)` |

## What I rely on (tell me in your notes if any of it changes)

- `normalizeHandle`, `handleActorId`, `HANDLE_MAX`, `HANDLE_INPUT_MAX` exported from `@semio-tech/quiz`.
- A refused `handle` query answers HTTP 400 whose error message contains the token `handle-invalid`; an id that fails answers with
  `id-invalid` in the rejection detail or the error message.
- `leaderboard(standings, caller?)`, `standing(state, catalogView)`, `decideHandle`, `evolveHandle`, `emptyHandleState`, `decideLearner`
  with `context.limits` (`DEFAULT_LIMITS`) as they are in the working tree now.
- The proctor's `429` carries `Retry-After` (seconds); the client waits that long plus its own jittered backoff.

## Tests of mine that use a test directory emoji

`🌍️language-choice`, `📢️live-regions`, `🌗️contrast-states`, `📇️learner-pages`, `🚦️rate-limits`, `🎭️identity-step`, `🔏️privacy-notice`
(tests and fixtures) — yours is `🪪️identity-shapes`, no clash.

## Status

- 2026-10-02 01:20 — written; React target adaptation in progress.
- 2026-10-02 — final. The React target runs on your contract: checked in a browser against the real proctor
  (anonymous registration, pseudonym registration, recall of `  probe   FUCHS 71 ` as "Probe Fuchs 71" by the `handle`
  query alone, a refused handle explained at the field, a run submitted, the leaderboard page with `learners` and `own`).
  The react suite passes (16 files, 364 tests).
