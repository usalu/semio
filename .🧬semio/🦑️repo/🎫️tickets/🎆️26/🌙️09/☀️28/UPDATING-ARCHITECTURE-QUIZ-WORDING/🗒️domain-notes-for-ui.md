# 🗒️ Domain notes for the agent "ui"

Written by the agent "domain" (quiz cores, schema, proctor actors/projections/queries/cli). Contract: `🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json`; TS types in `🧬️schema/🟦️.ts`. Status line at the bottom says what has landed.

## 1. Leaderboard (coordinator's shape, implemented exactly)

```ts
type Leaderboard = {
  readonly rows: readonly LeaderboardRow[]; // the top rows by rank, at most 100 (LEADERBOARD_TOP)
  readonly learners: number;                // how many learners are ranked in total
  readonly own?: LeaderboardRow;            // the caller's row when the caller is a ranked learner, also when it is inside the top
};
```

- The caller is named **in the query**: `{ type: "leaderboard", learner?: Id }`. The proctor resolves every principal as anonymous (the gateway overwrites the envelope principal), so the envelope principal cannot carry it. `ProctorClient.leaderboard(learner)` must put the learner id into the query arguments.
- `own.rank` may be greater than 100; `own` is absent for a caller without a submitted run and for an unknown id.
- Summing `rows[].runs` no longer counts every submission (only the top 100). Use `learners` and `own` for "how many play"; for "did anything change" compare the whole answer.
- `LeaderboardRow.lastActivity` is now **when the learner last submitted a run** (it no longer moves on answers or recalls). Ranking rules are unchanged.
- Rows still carry `tag`, never a learner id.

## 2. Identification: register by command, recall by query

A recall no longer writes anything. The roster actor is gone.

| Step | Wire |
|---|---|
| Anonymous | command `identify-learner` with `identity: { kind: "anonymous" }`, **target** `{ kind: "quiz-learner", id: command.learner }` → `learner-registered`. A learner id that is already registered → rejection `learner-exists`. |
| Pseudonym / name, ask first | query `{ type: "handle", handle }` (kind `quiz.handle`) → `HandleView { display: string; holder?: { learner: Id; identity: Identity } }`. `holder` present = the handle is claimed: adopt `holder.learner` (that is the recall). A handle the policy refuses → HTTP 400 whose message contains `handle-invalid`. |
| Pseudonym / name, free handle | command `identify-learner`, **target** `{ kind: "quiz-handle", id: handleActorId(normalizeHandle(handle).key) }` → `learner-registered { learner, identity }` (identity carries the normalized display handle). |
| Lost the race | the same command → rejection `handle-claimed` → ask the `handle` query again and adopt its holder. |

- `learner-recalled` no longer exists (removed from `Event`).
- Core helpers (both twins): `normalizeHandle(raw) → { display, key } | undefined`, `handleActorId(key)` (lowercase hex of the key's UTF-8 bytes), `isId`, `isSlug`.
- Principal of every command and query stays as today (the gateway ignores it).

## 3. Handle policy (server-side, both cores bit-identical)

`normalizeHandle`: White_Space runs collapse to one space and are trimmed; `’` (U+2019) becomes `'`; then the display must be 1…64 code points of: Latin letters (A–Z, a–z, Latin-1, Latin Extended-A/B, Latin Extended Additional — upper- and lowercase letters only, no ligatures or compatibility forms), ASCII digits, single spaces between words and `'` `.` `_` `-`, with at least one letter or digit. Everything else is `handle-invalid`: control and format characters, zero-width and bidi characters, combining marks (so **NFD input is refused — keep sending `handle.normalize("NFC")`**), other scripts (Cyrillic and Greek look-alikes), emoji, raw input over 256 code points. The key is the lowercased display.
`Identity.handle` in events, views and presence states is always the normalized display (`$defs/Handle` has the pattern); a presence state whose handle is not in that form is refused (`handle-invalid`).

## 4. New rejections (`REJECTIONS` grows; `quizRejection` picks them up from the detail)

| Code | When |
|---|---|
| `id-invalid` | a command or query id is not 32 lowercase hex, or a quiz/task id is not a slug |
| `handle-claimed` | `identify-learner` for a handle another learner holds |
| `learner-exists` | anonymous `identify-learner` for a learner id that is already registered |
| `roster-full` | the proctor holds its maximum number of learners (default 10 000) |
| `runs-exhausted` | `start-run` after 200 submitted runs of that quiz or 1 000 in total |
| `answers-exhausted` | `record-answer` after 2 000 recorded answers of that run (the run can still be submitted) |

| `envelope-mismatch: …` (not in `REJECTIONS`, a prefix of the detail) | the envelope does not agree with its command: wrong version, tenant or scope, `commandId`/`idempotencyKey` not the command id, or a well-formed target that is not the command's own (for a named `identify-learner`: anything but `quiz-handle/<handleActorId(key)>`). A target of no quiz actor's shape is `id-invalid`. |

A refused query is HTTP 400 `{ kind: "badRequest", message }` whose message **starts with** the code: `id-invalid: the learner query is refused`, `handle-invalid: the handle query is refused`.

## 5. Things a client should know

- **One registration per learner id.** A learner id that already registered (anonymously or under a handle) and then claims a handle gets `learner-registered` from the handle actor and holds the handle — but its own stream keeps the identity it registered first (the learner view, the leaderboard row). Generate a fresh learner id for every new registration; never "upgrade" an anonymous id.
- **`roster-full` counts registrations**: anonymous learners plus claimed handles (default 10 000, `PROCTOR_MAX_LEARNERS`).
- **After an erasure** (`proctor erase`, operator-only) the learner id is unknown: `quiz.learner`/`quiz.run` answer 404 and commands `unknown-learner`; the handle is free again. A browser still holding that id should fall back to the identity screen.
- **Reading who holds a handle** is only the `handle` query; `GET /actors/<catalog>/quiz-handle/<id>/events` is 403.

## 6. Edits I made in the React target

None. `🎯️targets/⚛️react` is untouched by me; it has to follow sections 1–4 (query argument `learner`, handle targets, recall by query, the new `Leaderboard` shape and rejections).

## Status

- 2026-10-02 00:40 — design fixed, implementation in progress (schema first, then both cores, then the proctor).
- 2026-10-02 — **landed.** Schema, both cores, conformance vectors and the proctor are in; everything above is what the running proctor does (`cargo test -p teaching-proctor` green including the end-to-end API tests; parity of the quiz product green). Report: `📓️domain-hardening-report.md` in this folder.
