# 🧑‍🏫 Quiz Product and Teaching Proctor — Design (normative)

Inputs: `📓️explore-*.md`, `📓️research-quiz-content-data.md` in this folder.
Contract: `🧰️framework/🛍️products/❓️quiz/🧬️schema/🔣️.json` (JSON Schema draft-07, normative; every language twin
matches it field for field — the repo's schema parity laws check this).

## 1. Tree and names

```
🧰️framework/🛍️products/❓️quiz/                              framework.product.quiz
  README.md                                                 domain model (like 🎤️presentation/README.md)
  🟦️.ts                                                     product barrel → 📦️packages/🟦️typescript/🟦️.ts
  🦀️.rs                                                     Rust product façade (like 🖥️server/🦀️.rs)
  🧬️schema/{🔣️.json,🟦️.ts,🦀️.rs}                          normative contract + hand-written type twins
  🔨️modules/<module>/{🟦️.ts,🦀️.rs}                         domain modules, TS and Rust twins side by side:
      🎲️randomness  🃏️sheet  ✅️validation  📏️scoring  🏅️badges  🧾️lifecycle  👁️views
  📦️packages/🟦️typescript/                                  @semio-tech/quiz: package glue only (barrel over 🧬️schema + 🔨️modules)
  🎯️targets/⚛️react/{🟦️.tsx,🎨️.css,🔨️modules/<m>/🟦️.ts(x)}  web renderer + proctor client (implementation at the target root)
  🎯️targets/⚛️react/📦️packages/🟦️typescript/               @semio-tech/quiz-react: glue only (package.json, 📋️project.json, 📜️script.ts, re-export 🟦️.tsx)
  📦️packages/🦀️rust/                                        crate semio-framework-quiz (lib quiz, #[path] glue), nx @semio-tech/quiz-rs
  🧪️tests/🎚️config/🟦️.ts                                     vitest config of the core
  🧪️tests/<case>/{🥒️.feature,🟦️.ts}                         language-agnostic cases (Rust twin under 📦️packages/🦀️rust)
  🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts                   vitest config of the react target (mirrored path, like presentation)
  🧫️fixtures/<case>/🔣️.json                                 shared vectors (inputs + expected outputs)
  🔮️oracles/🔣️.json                                         third-party oracles (schemaVersion 2 registry)

🎓️teaching/                                                 area (clean, implementation)
  README.md
  🛂️proctor/                                               Rust ServerInstance over one SQLite file
    README.md, 📦️packages/🦀️rust (crate teaching-proctor, bin proctor, nx @teaching/proctor)
    🧪️tests/…, Dockerfile-less: deployment lives with the site that deploys it
  🏛️architecture/
    ❓️quiz/                                                 catalog + website quizze.architektur-und-technologie.de
      🔣️.json                                              Catalog (intro, quiz paths, badges)
      📦️packages/🟦️typescript/                              @teaching/architecture-quiz (vite app)
      🏗️builder/🌐️vite/🟦️.ts
      🚀️deploy/{Dockerfile,compose.yaml,Caddyfile}          (names per taxonomy; see §9)
    ⚡️energy/
      🧲️physics/❓️quiz/🔣️.json                           Physikalisches Verständnis
      🔥️heating/❓️quiz/🔣️.json                           Heizen
      ❄️cooling/❓️quiz/🔣️.json                           Kühlen
      📊️demand/❓️quiz/🔣️.json                            Energiebedarf
```

`🎬️clip` siblings of each `❓️quiz` belong to the intended tree but are out of scope.

## 2. Identifiers

- Slugs: `^[a-z0-9]+(?:-[a-z0-9]+)*$`, unique per scope (quiz ids in a catalog, task ids in a quiz, item/category/axis/
  dimension ids in a task, badge ids in a catalog).
- Learner, run and command ids: 32 lowercase hex chars (128 random bits, `crypto.getRandomValues` on the client).
- Revision: lowercase hex SHA-256 of the quiz file bytes as loaded by the proctor.

## 3. Randomness (bit-exact across languages)

- `seed(run)`: FNV-1a 32-bit over the UTF-8 bytes of the run id string: `h = 2166136261; for b: h ^= b; h = (h * 16777619) mod 2³²`.
- PRNG: MT19937 (32-bit Mersenne Twister) seeded with `init_genrand(seed)`; `next()` yields the standard tempered u32
  (seed 5489 → first output 3499211612).
- `uniform(n)` for n ≥ 1: `n == 1 → 0`; else `limit = 2³² − (2³² mod n)`; draw `x = next()` until `x < limit`; return `x mod n`.
- `shuffle(a)`: Fisher–Yates from the end: for `i = len−1 … 1`: `j = uniform(i+1)`; swap `a[i]`, `a[j]`. In place on a copy.

## 4. Sheet: `sheet(quiz, seed)`

1. `rng = MT19937(seed)`.
2. `order = shuffle([0 … tasks.len−1])` — the presentation order of tasks.
3. For every task in definition order `t = 0 … n−1` (RNG consumption order matters):
   - `items = shuffle(copy(task.items))`; if `draw` is set and `draw < items.len`: `items = items[0 … draw−1]`.
   - classification: `categories = shuffle(copy(task.categories))`; axes copied unchanged.
   - sorting: if `len ≥ 2` and the item order equals the ascending order (stable by value, ties by definition index),
     rotate left by one (first item moves to the end). No RNG draw.
   - matching: for every dimension in definition order: `cards = shuffle(items.map(i → i.values[dimension]))`.
   - Sheet items carry only `{ id, label }`.
4. `sheet.tasks = order.map(t → sheetTask[t])`; `seed`, `quiz` (id), `title`, `description` copied.

## 5. Answers

- classification `{ assignments: item → category }`; sorting `{ order: [item] }`; matching `{ assignments: dimension →
  item → cardIndex }`.
- Valid (else `answer-invalid`): kind matches; every referenced item/category/dimension exists in the sheet task; card
  indices are in range; within a dimension no card index is used twice; a sorting order is a permutation of the sheet
  items. Partial classification/matching answers are valid while the run is open.
- Complete: classification assigns every sheet item; matching assigns every sheet item in every dimension; sorting is
  always complete once recorded. Submission requires a complete answer for every sheet task (`run-incomplete`).

## 6. Scoring (per task in [0, 1]; run score = mean of task scores in sheet order)

`s(v)` = `v` for `linear`, `log10(v)` for `logarithmic` (values must be > 0 — a validation issue otherwise).

- **Sorting — magnitude-weighted pair concordance.** Over the learner's order `o`, for `i < j` (i outer ascending, j
  inner ascending): `w = |s(v[o_i]) − s(v[o_j])|`; `total += w`; if `v[o_i] > v[o_j]`: `discordant += w`.
  `score = total > 0 ? 1 − discordant / total : 1`. Swapping near neighbours costs little, swapping a tea light and a
  nuclear plant costs a lot; perfect = 1, reversed = 0. With unit pair weights the score equals `(1 + τ_a) / 2`
  (Kendall, scipy oracle); with equally spaced ranks as values on a linear scale it equals `(1 + ρ) / 2` (Spearman,
  jStat oracle). (Corrected 2026-09-28: the earlier "s = rank ⇒ Kendall" claim is false for n ≥ 3.)
- **Matching** — per dimension, items in sheet order, `a_i = cards[assignment[item_i]]`, `t_i` the true value; for
  `i < j`: `w = |s(t_i) − s(t_j)|`; `total += w`; if `sign(t_i − t_j) · sign(a_i − a_j) < 0`: `discordant += w`, else if
  `a_i == a_j` and `t_i ≠ t_j`: `discordant += w / 2`. Dimension score as above; task score = mean over dimensions in
  definition order.
- **Classification** — items in sheet order; credit 1 when assigned == correct; else when both categories carry
  profiles: `max(0, 1 − d(assigned, correct) / d_max)` with `d` the Euclidean distance over the task axes (definition
  order) of normalised values `(v − min) / (max − min)`, and `d_max` the largest distance over all category pairs
  (definition order `i < j`) that both carry profiles (`d_max = 0 → credit 0`); else 0. Score = mean credit.
- Results: classification items in sheet order with `assigned`, `correct`, `credit`, `explanation`; sorting items in the
  learner's order with `value`, `position`, `rank` (position in the true ascending order, ties by definition index),
  `explanation`; matching per dimension items in sheet order with `assigned` value and `correct` value, `explanation`.
- Parity tolerance for scores across languages: 1e-12 (log10 may differ by one ulp). Perfect is exact (`discordant == 0`).

## 7. Badges

Evaluated after every submission, in catalog badge order, over all submitted results of the learner including the new
one; badges already held are skipped; a new award records the triggering run and timestamp.
- `perfect-quiz { quiz }`: some result of that quiz has score 1.
- `perfect-tasks { taskKind?, quiz? }`: every catalog task matching the selector (kind and/or quiz; none = all tasks)
  has score 1 in some result. A selector matching no task never awards.
- `completed-quizzes`: every catalog quiz has at least one submitted result.

## 8. Learner lifecycle (pure `decide`/`evolve`, shared by both cores; proctor wraps it in framework deciders)

Handles: server key = lowercase of the trimmed handle with inner whitespace runs collapsed to one space (the client
sends NFC); display = trimmed/collapsed handle; 1…64 chars else `handle-invalid`. Pseudonyms and names share one key
space.

| Command | Condition | Outcome |
|---|---|---|
| identify-learner anonymous | — | `learner-registered { learner (from command), identity }` |
| identify-learner pseudonym/name | key unclaimed | `learner-registered { learner (from command), identity (display handle) }` |
| identify-learner pseudonym/name | key claimed by L | `learner-recalled { learner: L }` |
| start-run | learner unknown | reject `unknown-learner` |
| start-run | quiz unknown | reject `unknown-quiz` |
| start-run | open run of quiz with current revision | reject `run-open` |
| start-run | open run of quiz with stale revision | `run-voided { old }`, `run-started { run, quiz, revision, seed(run) }` |
| start-run | otherwise | `run-started` |
| record-answer | run unknown / not open | `unknown-run` / `run-closed` |
| record-answer | stale revision | reject `quiz-revised` |
| record-answer | task not in sheet / answer invalid | `unknown-task` / `answer-invalid` |
| record-answer | otherwise | `answer-recorded` (the latest answer per task wins) |
| submit-run | run unknown / not open | `unknown-run` / `run-closed` |
| submit-run | stale revision | `run-voided` |
| submit-run | incomplete | reject `run-incomplete` |
| submit-run | otherwise | `run-submitted { result }`, then `badge-awarded` per newly earned badge |

`at` is the decision time (`DecisionContext.now` in ms). Commands are idempotent by command id.

## 9. Proctor

- Rust `ServerInstance` (see `📓️explore-server-product.md` §6): modules, deciders for actor kinds `roster` (single actor
  holding the handle index; identify-learner) and `learner` (one actor per learner; runs, answers, submissions, badges),
  a projection saga folding every event into the SQLite projections (learner views, run views, leaderboard), query
  handler for the four queries, the four storage traits implemented over one `rusqlite` (bundled) connection in WAL mode,
  held to the framework `conformance` suite.
- Principal: none required beyond the learner id carried in commands (identity without passwords by specification);
  the policy admits the four command kinds and four query kinds for every principal.
- Static hosting: the proctor serves the built site directory (`PROCTOR_SITE`) with SPA fallback to `index.html`.
- Configuration by environment only: `PROCTOR_PORT`, `PROCTOR_DATA` (directory of the SQLite file), `PROCTOR_CATALOG`
  (catalog `🔣️.json`), `PROCTOR_SITE`, `PROCTOR_MODE=production` + `PROCTOR_ALLOWED_ORIGINS` + trusted forwarding as hub.
- Deployment for `quizze.architektur-und-technologie.de`: two-stage Dockerfile (bun builds the site, cargo builds the
  proctor, runtime image `debian:bookworm-slim` + tini, one volume), compose service on `127.0.0.1:<port>`, Caddy
  (automatic TLS) reverse proxy.

### 9a. Wire mapping onto the framework server contract (`@semio-tech/framework-server` ↔ `semio-framework-server`)

| Framework field | Value |
|---|---|
| `CommandEnvelope.kind` / `version` | `quiz.<Command.type>` (e.g. `quiz.start-run`) / `1` |
| `CommandEnvelope.commandId`, `idempotencyKey` | `Command.id` |
| `CommandEnvelope.target` | identify-learner → `{ tenant: <catalog id>, kind: "quiz-roster", id: "roster" }`; the other three → `{ tenant: <catalog id>, kind: "quiz-learner", id: Command.learner }` |
| `CommandEnvelope.scope` | `<catalog id>` |
| `CommandEnvelope.principal` | `{ kind: "anonymous" }` before identification, `{ kind: "user", id: <learner> }` after |
| `CommandEnvelope.payload` | UTF-8 bytes of the `Command` JSON |
| `EventRecord.payload` | UTF-8 bytes of the `Event` JSON; `EventRecord.kind` = `quiz.<Event.type>` |
| `QueryEnvelope.kind` / `arguments` | `quiz.<Query.type>` / UTF-8 bytes of the `Query` JSON; consistency `authority` |
| `QueryResult` | `snapshot` whose `value` is UTF-8 bytes of the view JSON (`CatalogView`, `LearnerView`, `RunView`, `Leaderboard`) |
| Rejection | framework `Rejection` whose code/message carries the quiz `Rejection` string |

The proctor serves the gateway routes at the site origin root (`/instance`, `/commands`, `/queries`, …) and the site
for every other GET path (SPA fallback to `index.html`); the dev site (vite) proxies those routes to the dev proctor.

## 10. Client (react target)

- Steps: introduction (first visit) → identity (anonymous / pseudonym / name) → home (quizzes, best scores, badges) →
  run (tasks in sheet order, progress, submit when complete) → results → leaderboard (all learners, total, per-quiz best,
  badges, runs).
- Interaction: every task works with keyboard only (move up/down, assign via select/listbox, card pickers) and pointer
  drag and drop; spider diagrams are owned SVG with text alternatives.
- Local-first: persisted local-only (learner id, locale, theme, outbox, cached catalog, cached run views), persisted
  shared (proctor events), ephemeral shared (leaderboard polling), ephemeral local-only (drag, focus, step). Answers
  apply locally at once; commands go through an outbox coalesced per (run, task), retried with
  `retryWithJitteredBackoff`, idempotent by command id; the connection state is visible; submission shows progress and
  can be cancelled.
- i18n: `en` and `de` everywhere (no default; browser language, English on ties; explicit choice persisted).
- Customization: theme (light/dark/system) and text size via CSS custom properties on the semio palette.

## 11. Core API (identical vocabulary in both cores; Rust uses snake_case)

Types: one TS type / Rust type per schema `$defs` entry, same names (`Text`, `Quantity`, `Task`, `Quiz`, `Catalog`,
`Sheet`, `SheetTask`, `Answer`, `TaskResult`, `RunResult`, `Identity`, `Command`, `Event`, `Rejection`, `CatalogView`,
`LearnerView`, `RunView`, `Leaderboard`, `Query`, …). Rust: serde with `rename_all = "camelCase"` fields, internally
tagged enums (`kind` for tasks/sheet tasks/answers/results/identity/badge rules, `type` for commands/events/queries),
kebab-case variant names, `BTreeMap` for id-keyed maps, `f64` values, `u32` seeds, `u64` timestamps.

| TS (`@semio-tech/quiz`) | Rust (`quiz` crate) | Contract |
|---|---|---|
| `fnv1a32(text)`, `runSeed(run)` | `fnv1a32`, `run_seed` | §3 |
| `new Mt19937(seed).next()` | `Mt19937::new(seed).next_u32()` | §3 |
| `uniformIndex(random, n)`, `shuffle(random, items)` | `uniform_index`, `shuffle` | §3 |
| `sheetOf(quiz, seed)` | `sheet_of` | §4 |
| `quizIssues(quiz)`, `catalogIssues(catalog, quizzes)` → `{ path, code }[]` | `quiz_issues`, `catalog_issues` | structural + semantic validation without any schema library |
| `answerRejection(sheetTask, answer)`, `answerComplete(sheetTask, answer?)` | `answer_rejection`, `answer_complete` | §5 |
| `scoreTask(task, sheetTask, answer)`, `scoreRun(quiz, sheet, answers)` | `score_task`, `score_run` | §6 |
| `earnedBadges(badges, quizzes, results, held)` | `earned_badges` | §7 |
| `normalizeHandle(handle)` → `{ display, key } \| undefined` | `normalize_handle` | §8 |
| `RosterState`, `decideRoster(state, command, now)`, `evolveRoster(state, event)` | `decide_roster`, `evolve_roster` | §8 |
| `LearnerState`, `emptyLearnerState(learner)`, `decideLearner(state, command, context)`, `evolveLearner(state, event)` | `decide_learner`, `evolve_learner` | §8; `context = { now, catalog, quizzes: id → { quiz, revision } }`; decision = `{ events }` or `{ rejection }` |
| `catalogView(catalog, quizzes)`, `learnerView(state, catalog)`, `runView(state, run, quizzes)`, `leaderboard(states, catalog)` | `catalog_view`, `learner_view`, `run_view`, `leaderboard` | schema views |

Shared vectors live in `🧰️framework/🛍️products/❓️quiz/🧫️fixtures/<case>/🔣️.json` and are read by both cores' unit
tests and by the Protocol v2 cases.

## 12. Seams between work packages

- Dev ports: site (vite) `6061` (`TEACHING_ARCHITECTURE_QUIZ_PORT`), proctor `8791` (`PROCTOR_PORT`); the dev site
  proxies `/instance`, `/commands`, `/queries`, `/actors`, `/scopes` to `http://127.0.0.1:8791`.
- Renderer entry (`@semio-tech/quiz-react`): `mountQuiz(root: HTMLElement, options: { proctor: string; tenant: string }):
  () => void` — `proctor` is the base URL (`""` = same origin), `tenant` the catalog id; returns the unmount function.
  Also exports the `QuizApp` React component with the same options as props.
- Proctor binary: `proctor` (crate `teaching-proctor`), `proctor serve` reads the §9 environment; `proctor check
  <catalog>` validates a catalog and its quizzes and exits non-zero with the issues.
- Catalog of the site: `🎓️teaching/🏛️architecture/❓️quiz/🔣️.json` (id `architecture`), quizzes by relative path.
- Shared registrations: products `🔣️.json` member and the core workspace entry → core-ts; Cargo workspace member +
  `[workspace.dependencies]` entry → core-rust and proctor (their own lines); react workspace entry → react; site
  workspace entry, root scripts, launch rows (`.vscode/launch.json` + seed), taxonomy (`🔣️taxonomy.json`) for every
  new directory → site-infra; oracle registry → conformance.

## 13. Revisions after audit (2026-09-28)

- `LeaderboardRow.learner` → `tag` = `fnv1a32(learner id)` as 8 lowercase hex digits. The proctor has no passwords
  and resolves every request as anonymous, so a learner id is the only credential of an anonymous learner; the public
  leaderboard must not publish it. Ordering keeps the learner id as the last internal tie-break.
- Both cores must degrade identically on inputs that bypass validation (no NaN, no throw): empty means are 0,
  incomplete profiles are excluded from the profiled set, invalid answers score to "none" → `run-incomplete`.
- The React target lives at `❓️quiz/🎯️targets/⚛️react/` with package glue under its own `📦️packages/🟦️typescript/`.

## 14. Revision 2026-09-29 — card-grid UI, CDN site, zero-touch proctor

Inputs: `📓️ui-reference-play-demonstrator.md` (prototype `🗑️generated/ui-reference/rig/quiz-mock.tsx`),
`📓️explore-cdn-docker-deploy.md`.

### Domains and split
- Site: static build on a CDN at `https://quizzes.architektur-und-technologie.de` (GitHub-Pages-shaped artifact:
  `index.html`, `404.html`, `CNAME`, `.nojekyll`, `_headers`; `base: "/"`).
- Proctor: API only on Docker at `https://proctor.quizzes.architektur-und-technologie.de`. The proctor no longer hosts
  the site (`🌐️site` module and `PROCTOR_SITE` removed); CORS allowlist = the site origin; preflights carry
  `Access-Control-Max-Age`.
- Client origin: a release build bakes `import.meta.env.VITE_PROCTOR_URL` (default the proctor origin above,
  overridable by `PROCTOR_URL` at build time); dev and tests keep `""` (same origin via the Vite proxy).
- Old host `quizze.…` is renamed everywhere ("Quizze" as the German word stays).

### Zero-touch proctor on Docker
- Proctor-only image (no bun stage), production defaults baked in (`PROCTOR_MODE=production`,
  `PROCTOR_ALLOWED_ORIGINS=https://quizzes.architektur-und-technologie.de`, `PROCTOR_TRUSTED_FORWARDING=proxy`,
  catalog/data paths, bind/port), `HEALTHCHECK`, tini, non-root, one volume.
- `compose.yaml` = proctor (`expose` only) + Caddy service (automatic TLS for `{$PROCTOR_HOST:proctor.quizzes…}`),
  named volumes for proctor data and Caddy data/config, `restart: unless-stopped`, Caddy `depends_on` healthy proctor.
  The whole host contract: DNS A/AAAA + ports 80/443(+udp) + `docker compose up -d`.
- Image published to GHCR (`ghcr.io/usalu/…`) by an explicit publish verb and a manual `workflow_dispatch` workflow;
  compose keeps a `build:` block so a host can also build from a clone.

### CDN site
- `publish` verb: build with the production proctor origin, verify the artifact (markers, CNAME = site host, proctor
  origin baked, no localhost), stage it; a manual `workflow_dispatch` workflow deploys it to GitHub Pages (Fastly CDN);
  the artifact stays portable to any static CDN.

### UI: card grid like semio-tech play / mit-bestand demonstrator
- Every card is the design system's `WindowChrome` (`level="dialog"`), chips from `windowChromeTitleChipClass`, header
  `Navbar` + `ShellBrandLogo`, icons `Icon`, the same CSS token chain as play/demonstrator (sharp corners, glass, 1 px
  outline, Anta, palette, light/dark via the shared appearance helpers).
- The play/demonstrator card is extracted once as a shared ui element (overview card) that the quiz, play and the
  demonstrator all use (no third copy).
- Home = a grid of nine sections in DOM/reading order: learner, quiz 1, how it works, quiz 2, **leaderboard (centre)**,
  quiz 3, badges, quiz 4, preferences. Desktop ≥ 1024: 3 × 3 with the larger centre cell; tablet 768–1023: 2 columns
  with the leaderboard spanning the middle row; phone ≤ 767: one column.
- Leaderboard card: top rows + own row (`aria-current`) by `tag`; "Full leaderboard" opens the full sortable table.
- Quiz card: emoji + title, description, task count, best score, run state, earned quiz badge; actions Start / Resume /
  Play again and Last result as real buttons.
- Bundle budget: import only slim ui-react subpaths (as `@semio-tech/ui-react/i18n`); main chunk stays near today's
  414 kB.
- Schema: `Quiz.emoji` and `CatalogQuizView.emoji` (required).

## 15. Revision 2026-09-29 — shared presence and cursors (ephemeral shared state)

Requirement: the presence of every learner is shared with the other learners, including the cursor.

### Framework (domain-neutral, `🧰️framework/🛍️products/🖥️server`)
- New gateway route `GET /scopes/{scope}/presence/ws?surface=<≤64 chars>` (WebSocket, subprotocol
  `semio.presence.v1`, text JSON frames), added to `base_router` and the wire fixture route table; TS client twin in
  `@semio-tech/framework-server` (`presenceSocketUrl` + frame codec).
- Latest-state semantics, coalesced per tick: the server keeps each session's latest `state` (opaque JSON) and every
  tick (default 100 ms) publishes one `batch` of the sessions that changed plus those that left. Traffic per client is
  O(changed sessions) per tick, independent of how often each client moves.
- Frames:
  - server → client on join: `{ "type": "welcome", "session", "colour", "roster": [{ "session", "colour", "surface", "state" }] }`
  - client → server: `{ "type": "state", "state": <json> }` (≤ 2 KiB serialized, ≤ 30 per second; excess dropped)
  - server → client per tick with changes: `{ "type": "batch", "entries": [{ "session", "colour", "surface", "state" }], "left": ["session", …] }`
  - server → client on a refused state: `{ "type": "refused", "reason" }` (the socket stays open)
- Sessions are server-generated random ids; colours come from the existing `Presence` palette slots; join/leave go
  through the existing `Presence` registry; ping/pong keepalive, idle timeout 60 s.
- Admission: `PolicyPoint::Subscription` (resource `presence`, actions `join`/`publish`); every `ServerModule` may
  validate a state through a new default method `presence_admission(scope, state) -> Result<(), String>` (default
  accepts), so existing instances (hub) are unaffected. In production the socket's `Origin` must pass the instance's
  origin allowlist.

### Quiz (schema `Place`, `Anchor`, `Cursor`, `PresenceState`, `CursorState`; both cores validate)
- Rooms: roster scope `<catalog>` carries `PresenceState` (tag, identity, place, active; changes on navigation and
  visibility); place scopes carry `CursorState` (tag, cursor, focus): `<catalog>/home`, `<catalog>/leaderboard`,
  `<catalog>/introduction`, `<catalog>/quiz/<quiz>` (run and results of one quiz). Core helpers: `rosterScope(catalog)`,
  `roomScope(catalog, place)`, `presenceProblem(state)`, `cursorProblem(state)` (Rust twins), and the proctor's
  `presence_admission` uses them (scope decides the type).
- Cursor positions are relative (0…1) to an `Anchor` every learner in the room renders (home cards, leaderboard, task
  card `task:<id>`), so they survive viewport differences and randomized item orders; items, cards and drag targets are
  never shared, so presence reveals no answers.
- Client: two sockets (roster + current room), reconnect with jittered backoff, pointer frames throttled to ≤ 15 Hz and
  coalesced (latest wins), keyboard focus shared as `focus` anchor (outlined card for keyboard users), others' cursors
  interpolated unless `prefers-reduced-motion`, decorative (`aria-hidden`) with a text roster ("N online", who is where)
  for assistive technology, colour from the server slot, label = identity display (anonymous → "Anonymous #tag").
  Preference "Show others' cursors" (persisted local-only). Presence shows on the learner card/navbar (online count
  and list), on quiz cards (learners in this quiz now) and as online dots in the leaderboard.
- Decision 2026-09-29: the identity screen has no room (a learner there has no tag yet); place rules
  `task-without-run` (task only on run) and `quiz-outside-run` (quiz only on run/results) hold in both cores.

## 16. Revision 2026-09-29 — layered home like semio-tech play

Requirement: the quizzes page is multilayered like semio-tech play — the content of every card is rendered behind a
glassy layer; hovering (or keyboard-focusing) a card shows its page clear. Normative reference:
`📓️ui-reference-layered-landing.md` (§10 API and behaviour; prototypes `layered_overview_prototype.tsx`,
`layered_quiz_home_prototype.tsx` in the ticket folder).

- One shared domain-neutral element `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🥞️LayeredOverview` + pure geometry/policy
  module `🔨️modules/🥞️layered-overview-geometry` (exported via `@semio-tech/ui-react/chrome`): strip of panes (percent
  geometry, imperative transform), ONE `ui-veil` with a `clip-path` hole, card overlay, chrome, reveal on hover/focus
  and conceal on leave/blur, 500 ms eased glide, pointer pan, reduced motion, hash open/Escape/Overview with focus
  management, warm queue + budget + time-based release, posters only via an explicit poster function, list mode for
  touch phones, per-pane error boundary, windowing. Language-agnostic fixture + third-party oracle (`polygon-clipping`
  for the veil polygon).
- Play and the demonstrator migrate onto it (their duplicated ~620/660 lines disappear; the landing defects listed in
  the reference §10.9 are fixed on the way).
- Quiz home: nine panes in DOM/reading order (learner, physics, how it works, heating, leaderboard, cooling, badges,
  demand, preferences), each pane = the real page its card opens: learner profile, the read-only quiz page (new screen
  `quiz`; never a run — starting a run is a command), introduction, full leaderboard, badges page, preferences page.
  Compact cards centred in their cells; desktop 3 × 3 cells, tablet 2 × 5, phone list mode; the header stays in flow
  above the element. Backdrop pages are pure views over session state; polling only while opened or revealed.
- Schema `Screen` gains `quiz`, `learner`, `badges`, `preferences`. Presence rooms: `quiz` → `<catalog>/quiz/<quiz>`
  (quiz required; quiz allowed on quiz/run/results), `badges` → `<catalog>/badges`, `learner` and `preferences` → no
  room (personal pages); existing rules otherwise unchanged.

## 17. Revision 2026-09-29 (night) — sharing within the quizzes and a live grid backdrop

Requirements: presence is shared within the quizzes too — the quizzes are for fun and learners see what the others
think; the home page shows the real subpages behind the cards as a grid, live (presence of other learners inside
them, a leaderboard that keeps updating, …). This supersedes the §15 rule "never share answers or drags".

### What the others think
- **Live (ephemeral shared)**: thinking room `<catalog>/quiz/<quiz>/thinking` with `ThinkingState { tag, answers }` —
  the learner's current draft answers per task of the open run, published (coalesced, ≤ 2 Hz) whenever an answer
  changes; peers aggregate by item id (sheets differ per learner, so everything is semantic, never positional):
  classification → avatars/counts per category, sorting → others' normalized positions as markers, matching → others'
  assigned values. The quiz room's `CursorState` may anchor to items (`item:<id>`) and categories (`category:<id>`) and
  carries `drag { item }`, so peers see what someone is dragging and where they point.
- **Persisted (projection)**: `CrowdView` per quiz from all `run-submitted` results (classification category counts,
  sorting mean normalized position, matching value counts; tasks/dimensions/items in definition order, keys ascending);
  query `{ type: "crowd", quiz }` (`quiz.crowd`). Shown on the quiz page ("what others answered"), during the run as the
  crowd layer when nobody else is online, and on the results page (you vs. the crowd). Core function `crowdView(quiz,
  results)` in both cores (conformance vectors); the proctor keeps it as a projection.
- Both cores: `thinkingScope(catalog, quiz)`, `thinkingProblem(state)` (structural answer checks, size bounds), item and
  category anchors valid in cursor states, `drag.item` a slug.

### Live grid backdrop (home)
- `LayeredOverview` gains a rest mode `"grid"`: at rest the strip is scaled to fit, so all panes are visible as a grid,
  each exactly behind its card cell (the contact sheet of live pages), under the glass veil; revealing a card zooms and
  glides its pane to full size (veil hole as before); leaving zooms back out. `"panorama"` (play/demonstrator) stays.
- Backdrop pages are live: the leaderboard polls while the home is visible (not only when revealed); pages render other
  learners' presence inside themselves (their cursors on that page, online marks, learning-now counts, the thinking
  crowd on quiz pages).
- Framework presence socket gains **watch**: `{ "type": "watch", "scopes": [...], "intervalMs": n }` (≤ 16 scopes,
  interval ≥ tick; replaces the watch set) → server sends `{ "type": "watched", "scope", "entries", "left",
  "snapshot"? }` coalesced per interval, read-only (states are published only to the joined scope). Admission per
  watched scope through the same instance hook as joins (`PolicyPoint::Subscription`, action `watch`). The home joins
  its room and watches the page rooms (introduction, leaderboard, badges, every quiz room and thinking room) at ~4 Hz.
- Decisions 2026-09-29 (night): `ThinkingState.answers` holds `ThinkingAnswer`s — classification and sorting as in
  `Answer`, matching as `ThinkingMatchingAnswer { values: dimension → item → value }` (semantic; the publisher maps its own
  card indices to values), so peers can see the values others matched. Crowd count keys sort by code point in every
  implementation (deterministic contract); clients sort values numerically for display.
- Thinking-state bounds (both cores, conformance): at most 64 tasks per state and 64 entries per answer or dimension
  (`too-many`), no repeated item in a sorting draft (`duplicate-id`), matching values finite numbers (`type-invalid`),
  card indices refused in drafts; the presence socket's 2 KiB state cap applies on top.
