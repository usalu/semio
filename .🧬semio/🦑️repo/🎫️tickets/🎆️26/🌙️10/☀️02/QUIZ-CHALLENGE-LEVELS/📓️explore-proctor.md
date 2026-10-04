# Proctor map for the challenge levels (read-only exploration, 2026-10-02)

Scope read: `🎓️teaching/🛂️proctor/` (all Rust modules, README, bootstrap, script, project.json, tests), the quiz core
crate it calls (`🧰️framework/🛍️products/❓️quiz`: schema, lifecycle, sheet, randomness, scoring, badges, views,
validation), the framework server pieces it builds on (`🧰️framework/🛍️products/🖥️server`: authority, gateway), the
client's deputy, session and proctor client, and the design documents (`QUIZ-PRODUCT-AND-TEACHING-PROCTOR/📓️design.md`
§2–§9a, §17, §18, §20, `QUIZ-PERIOD-LEADERBOARDS/📓️design.md`).

Legend: **[V]** verified by reading the cited line; **[I]** inference or recommendation, not verified at runtime. No
cargo, no test, no build was run (as instructed). Working tree state at reading time: `git status` was clean for
`🎓️teaching/🛂️proctor` and for the quiz core `🔨️modules` and `🧬️schema` (other sessions edit the React target).

Paths below are relative to `🎓️teaching/🛂️proctor/` unless they start with `Q/` = `🧰️framework/🛍️products/❓️quiz/`,
`S/` = `🧰️framework/🛍️products/🖥️server/`. Module folders are `🔨️modules/<name>/🦀️.rs`; unit tests sit in
`🔨️modules/<name>/🧪️tests/🔬️unit/🦀️.rs`.

---

## 0. The ten facts that matter most

1. **The client picks the run id, and the run id is the seed.** `Command::StartRun { id, learner, run, quiz }`
   (`Q/🧬️schema/🦀️.rs:594`) has no seed and no level; the client generates `run: newId()`
   (`Q/🎯️targets/⚛️react/🔨️modules/🧭️session/🟦️.ts:802`); the decider sets `seed: run_seed(run)` = FNV-1a of the id
   (`Q/🔨️modules/🧾️lifecycle/🦀️.rs:191`, `Q/🔨️modules/🎲️randomness/🦀️.rs:22`). **[V]**
2. **The server never sends "the sheet" with a command answer.** A `start-run` answers only the events
   (`run-started` with `revision` and `seed`). The sheet is read afterwards with the query `quiz.run {run}`, which
   returns a stored `RunView` whose `sheet` the projector computed with `sheet_of(current quiz, seed)`
   (`Q/🔨️modules/👁️views/🦀️.rs:63-77`, stored by `🔨️modules/🔭️projections/🦀️.rs:458-462`). The client could also
   compute the very same sheet itself from `material` + seed (the deputy does, `Q/…/🫡️deputy/🟦️.ts:129-132,151`). **[V]**
3. **The sheet is solution-free by construction**: `SheetItem {id,label,icon}` carries no value
   (`Q/🧬️schema/🦀️.rs:334`); a matching `SheetDimension.cards` is the shuffled multiset of the true values
   (`:378`). An e2e assertion pins it: `assert!(!open["sheet"].to_string().contains("\"value\""))`
   (`🧪️tests/🌐️end-to-end/🦀️.rs:421`, and for the catalog `:388`). Showing sorting keys on "easy" needs a level-aware
   sheet, and that assertion must become level-aware. **[V]**
4. **Trust boundary: the client already holds every solution.** The architecture site bundles
   `ARCHITECTURE_QUIZ_MATERIAL` = catalog + the four quiz JSONs with `value`s into its script
   (`🎓️teaching/🏛️architecture/❓️quiz/📚️catalog/🟦️.ts:10-18`); the deputy decides locally with the core's own
   deciders while the proctor is away (`Q/README.md:489-528`). The documented guarantee is only: "The proctor still
   scores every run itself, so a leaderboard never trusts what a device computed" (`Q/README.md:527`). So *hidden values
   are a UI property, not a secret*, whatever the server withholds. The server can only guarantee scoring, level
   labelling, timing and ranking. **[V]**
5. **There is no deadline or duration logic anywhere** (grep over `Q/` and the proctor: only the capacity gate's
   throttle deadlines). The only clock is the server wall clock, stamped per command at the command lane
   (`S/🔨️modules/📡️gateway/🦀️.rs:2054` `state.now()`, `:1923-1926` HLC from `SystemTime`) and delivered to deciders
   as `context.now.millis` (`🔨️modules/🎭️actors/🦀️.rs:370`), becoming every event's `at`. The envelope's
   `clientHlc` is **never read** by the authority (only declared at `S/🔨️modules/🧬️contract/🦀️.rs:103`; grep
   `client_hlc` finds nothing else). **[V]**
6. **Server time of an answer is its arrival time, not the time the learner answered.** Answers travel through a
   client outbox (coalesced per run+task, retried with jittered backoff) and, while the proctor is away, are decided
   provisionally by the deputy and replayed later (`Q/README.md:486-521`). A per-question timer measured from server
   `at` is therefore unfair under connection shortages unless the design adds a server fact for "question opened" and
   a grace/ordering rule. **[V for the mechanics; I for the consequence]**
7. **Nothing expires a run.** An abandoned open run stays open forever; `start_run` refuses a new run of the same quiz
   with `run-open` while an open run of the *current revision* exists, and only a *stale* revision gets voided
   (`Q/🔨️modules/🧾️lifecycle/🦀️.rs:183-195`). The client answers `run-open` by *resuming* the old run
   (`…/🧭️session/🟦️.ts:804-807`). Expert timeouts and "start at another level while a run is open" both need an
   explicit voiding path. **[V]**
8. **A learner's score for a leaderboard is `best × 100` per quiz, summed over the catalog quizzes in catalog order**
   (`Q/🔨️modules/👁️views/🦀️.rs:422-424`), where `best` = maximum submitted score per quiz over the runs in the board's
   scope (`:410-420`). Ordering: total desc, badge count desc, `reachedAt` asc, learner id asc (`:178-193`). Level
   weighting must change `TranscriptRun`, `bests`, `total`, `LeaderboardRow.best` (a `Score` map) and the Python/TS
   twins; the proctor's `Board` just reuses these functions. **[V]**
9. **Formats:** `RunResult` has no level, `RunSubmitted` has no level, `TranscriptRun {quiz,score,at}` has none, the
   crowd tally is keyed by quiz only. Every type is `deny_unknown_fields`, so adding a field is a coordinated change of
   Rust schema + JSON Schema + TS twin + Python references + fixtures. Optional fields with `#[serde(default)]` keep old
   events decodable. The dev launcher moves old dev data aside **only** when the storage *format row* differs
   (`🏗️bootstrap/🟦️.ts:133-150`), not when `PROJECTOR_REVISION`/`STATE_FORMAT` change. **[V]**
10. **Surprise: `quiz.crowd` and the presence "thinking" rooms are the real solution leaks of a harder level.** The
    crowd's sorting `meanPosition`/`places` and matching value counts are readable any time ("the proctor withholds
    nothing", `README.md:466-467`); thinking drafts of peers carry sorting orders and matching *values*
    (`Q/🧬️schema/🦀️.rs:1085-1107`). Gating them per level can only be done on the server, and the query carries a
    claimed `learner` id which is the bearer credential (precedent: leaderboard `own`). **[V for the data; I for the fix]**

---

## 1. Commands, events, queries, projections

### 1.1 Request path of a command (outside in) **[V]**

`POST /commands` with a `CommandEnvelope` →
1. request gate (transport trust, CORS) `🔨️modules/🧩️instance/🦀️.rs:633-647`;
2. framework throttle (token per client address, `429/503`), body cap 16384 bytes;
3. `ServerModule::command_allowance` — only `quiz.identify-learner` spends a sign-up token
   (`🔨️modules/🧩️instance/🦀️.rs:193-195`, `SIGN_UP = "sign-up"` `🔨️modules/🎚️config/🦀️.rs:116`);
4. `ServerModule::command_admission` → `Admission::admit` (`🔨️modules/🎭️actors/🦀️.rs:161-166`) →
   `admitted()` (`:197-222`): version 1, tenant = scope = catalog id, target shape (`addressable` `:185`),
   payload decodes as `quiz::Command` (else `command-malformed: …`), `envelope.kind == "quiz.<type>"`,
   `quiz::command_rejection` (id shapes → `id-invalid`, `Q/🔨️modules/✅️validation/🦀️.rs:234-242`),
   `commandId == idempotencyKey == command.id`, target equals `command_target(...)`; for `IDENTIFY` also the learner
   cap (`roster-full`, `:170-176`);
5. policy (`learner_template`, `🔨️modules/🧩️instance/🦀️.rs:232-240`; every caller is `anonymous`);
6. idempotency receipt lookup (per principal+target), then the actor's turn: `LearnerDecider::decide`
   (`🔨️modules/🎭️actors/🦀️.rs:355-375`) which calls the pure `quiz::decide_learner` with
   `LearnerContext { now: context.now.millis, catalog, quizzes: self.catalog.current(), limits }`
   (`Q/🔨️modules/🧾️lifecycle/🦀️.rs:138-143`);
7. events are re-stamped by the authority (`stream, seq, hlc`; only `kind` and `payload` survive,
   `S/🔨️modules/🎭️authority/🦀️.rs:117,685-688`), appended with outbox rows, folded into the actor state by
   `evolve_learner`, snapshot every 64 events.

Rejections are `Rejection::Invalid { detail: "<quiz rejection string>" }` (`🔨️modules/🎭️actors/🦀️.rs:111-113`);
the client recovers the quiz rejection by regex over `notices`/`detail` and the fixed list `REJECTIONS`
(`Q/🧬️schema/🟦️.ts:276`, `…/🛂️proctor/🟦️.ts:299-316`). **A new rejection string must be added to the TS
`REJECTIONS`**, otherwise the client sees `refused`.

### 1.2 Commands (`Q/🧬️schema/🦀️.rs:587-597`) **[V]**

```rust
#[serde(tag = "type", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub enum Command {
    IdentifyLearner { id: Id, learner: Id, identity: IdentityClaim },
    StartRun { id: Id, learner: Id, run: Id, quiz: Slug },
    RecordAnswer { id: Id, learner: Id, run: Id, task: Slug, answer: Answer },
    SubmitRun { id: Id, learner: Id, run: Id },
}
```

| Wire kind (`quiz.<type>`) | Actor / stream | Decided by | Validation and outcome |
|---|---|---|---|
| `quiz.identify-learner` (anonymous) | `quiz-learner/<learner>` | `decide_learner` → `register` (`lifecycle:165-173`) | non-anonymous → `handle-invalid`; second time → `learner-exists`; emits `learner-registered`; admission also `roster-full` |
| `quiz.identify-learner` (pseudonym/name) | `quiz-handle/<hex of lowercase UTF-8 key>` | `decide_handle` (`lifecycle:53-71`) | normalize handle (`handle-invalid`), `handle-claimed` if held; emits `learner-registered` with the display spelling into the **handle** stream; the `EnrollmentSaga` relays it (see 1.5) |
| `quiz.start-run` | `quiz-learner/<learner>` | `start_run` (`lifecycle:175-196`) | needs registered learner (`unknown-learner`), quiz in catalog (`unknown-quiz`), run id unused (`run-open`/`run-closed`), no open run of the same quiz at the current revision (`run-open`; an open run of a stale revision is voided), caps `runs-exhausted`. Emits `[run-voided?, run-started{revision,seed=run_seed(run),at}]` |
| `quiz.record-answer` | learner | `record_answer` (`lifecycle:211-223`) | run open (`unknown-run`/`run-closed`), quiz revision unchanged (`quiz-revised`), `answers-exhausted` (2000/run, every change counts), task in the recomputed sheet (`unknown-task`), `answer_rejection` (`answer-invalid`). Emits `answer-recorded{task,answer,at}`; latest answer per task wins |
| `quiz.submit-run` | learner | `submit_run` (`lifecycle:225-243`) | run open; revision stale → emits `run-voided`; every sheet task `answer_complete` else `run-incomplete`; `score_run`; emits `run-submitted{result,at}` then one `badge-awarded` per newly earned badge |
| `quiz.enroll-learner` | learner | `LearnerDecider` (`actors:361-366`) | proctor-internal, only from service account `proctor`, relays the handle's registration (`enrolled()` `actors:226-245`) |

Manifest: `manifest()` `🔨️modules/🧩️instance/🦀️.rs:201-217` (commands, six queries, the eight projection names, three
policy templates, actor kinds `quiz-handle`/`quiz-learner`). `OfflinePolicy`: identify/start/submit
`AuthorityRequired`, record-answer `Optimistic` (`:206-210`). All kinds are version `WIRE_VERSION = 1`.

### 1.3 Events (`Q/🧬️schema/🦀️.rs:671-681`, stored as `quiz.<type>` with the JSON as payload) **[V]**

```rust
pub enum Event {
    LearnerRegistered { learner: Id, identity: Identity, at: Timestamp },
    RunStarted   { learner: Id, run: Id, quiz: Slug, revision: String, seed: u32, at: Timestamp },
    RunVoided    { learner: Id, run: Id, at: Timestamp },
    AnswerRecorded { learner: Id, run: Id, task: Slug, answer: Answer, at: Timestamp },
    RunSubmitted { learner: Id, run: Id, result: RunResult, at: Timestamp },
    BadgeAwarded { learner: Id, badge: Slug, run: Id, at: Timestamp },
}
```

`at` = `context.now.millis` (server HLC millis), also `EventRecord.hlc`. Streams: `quiz-learner/<id>` for all six
kinds; `quiz-handle/<hex>` holds exactly one `learner-registered` (a pseudonym/name claim). Stored in
`proctor_event(position, tenant, kind, id, seq, millis, counter, event_kind, payload)`
(`🔨️modules/🗄️storage/🦀️.rs:133`). `RunState`/`LearnerState` (the actor state bytes, `lifecycle:85-110`) are JSON of the
core types; `STATE_FORMAT = 1` (`actors:79`).

### 1.4 Queries (`🔨️modules/❓️queries/🦀️.rs`, `Q/🧬️schema/🦀️.rs:910-935`) **[V]**

`POST /queries`, `arguments` = bytes of `Query` JSON, answer `QueryResult::Snapshot { value: <view JSON bytes> }`.
Validation before any lookup (`query_rejection` → `400 id-invalid|handle-invalid`; unknown id/quiz → `404`). Handler
`QuizQuery::handle` at `🔨️modules/❓️queries/🦀️.rs:77-109`.

| Wire kind | Query JSON | Response shape | Source |
|---|---|---|---|
| `quiz.catalog` | `{type:"catalog"}` | `CatalogView` (solution-free: quizzes with task id/kind/title/icon, badges without rules) | precomputed bytes from `LoadedCatalog::view()` (`instance:479`) |
| `quiz.learner` | `{type:"learner",learner}` | `LearnerView { learner, identity, runs: RunSummary[] (newest first), badges, best: {quiz→Score}, total }` | projection `quiz.learner` |
| `quiz.run` | `{type:"run",run}` | `RunView { run, learner, quiz, status, sheet, answers, result?, startedAt, submittedAt? }` | projection `quiz.run` (no learner check; the 128-bit run id is the capability; it also exposes the `learner` id) |
| `quiz.leaderboard` | `{type:"leaderboard",period,quiz?,learner?}` | `Leaderboard { period, quiz?, window?, rows≤100, learners, submissions, own? }` | `Board::view(period, quiz, wall_clock(), learner)` |
| `quiz.crowd` | `{type:"crowd",quiz}` | `CrowdView { quiz, runs, scores[10], tasks[{task,kind,dimension?,scores[10],items[…]}] }` | projection `quiz.crowd` |
| `quiz.handle` | `{type:"handle",handle}` | `HandleView { display, holder?{learner,identity} }` | projection `quiz.handle` (writes nothing) |

Also the framework event-stream routes: `GET /actors/<catalog>/quiz-learner/<learner>/events[?since]` (+ `/events/ws`),
readable by the holder of the id; handle streams are policy-denied (`instance:237`, README 397-399).

### 1.5 Projections (`🔨️modules/🔭️projections/🦀️.rs`) **[V]**

| Name (const, line) | Key | Value | Rewritten by |
|---|---|---|---|
| `quiz.learner-state` `STATES` (68) | learner id | `Folded { seq, state: LearnerState }` | every learner event |
| `quiz.learner` `LEARNERS` (70) | learner id | `LearnerView` | every event except `answer-recorded` (`:439-441`) |
| `quiz.run` `RUNS` (72) | run id | `RunView` (sheet recomputed from the *current* quiz definition + seed, `run_view` `views:63-77`) | any event naming the run (`run_of` `:520-525`) |
| `quiz.leaderboard` `LEADERBOARD` (74) | learner id | `Transcript { learner, tag, identity, runs: TranscriptRun{quiz,score,at}[], badges: TranscriptBadge{badge,quiz,at}[] }` | registration, `run-submitted`, `badge-awarded` only (`:426-436,464-470`) |
| `quiz.handle` `HANDLES` (76) | handle key | `HandleHolder { learner, identity }` | `claim()` `:489-501` |
| `quiz.crowd-tally` `TALLIES` (80) | quiz id | `CrowdTally` (score bins, per-task/dimension bins, per-item counts, sorting `Positions`) | `run-submitted` (`:430-432`) |
| `quiz.crowd` `CROWDS` (78) | quiz id | `CrowdView` (`tally.view(quiz)`; empty views written for every quiz on `reset` `:348-350`) | `run-submitted` |
| `quiz.meta` `META` (82) | `catalog` → stamp `"{PROJECTOR_REVISION}:{fingerprint}"`; `learners` → registration count | | `prepare`/`reset`; `written` `:474-476` |
| checkpoint `quiz` | — | last folded log position | each batch of 256 (`BATCH`) in one transaction |

`PROJECTOR_REVISION: u32 = 5` (`:89`). The stamp mismatch (revision or catalog fingerprint) makes `prepare` reset all
read models and refold from event 1 (`:331-338`, README 308-312). The `Board` (in memory) = every transcript + one rank
index per `(period, quiz)` that was asked (`:120-276`).

**Folding is strict** (unknown stream kind, undecodable event, undecodable stored state/transcript → fold error naming the
position, `unreadable()` `:504-507`) and an actor with an undecodable stream/snapshot answers `actorUnavailable`
(`actors:261-321`). Consequence: a *required* new field (no `#[serde(default)]`) on `Event`/`RunState`/`Transcript`
makes every pre-existing dev database unreadable.

### 1.6 Enrollment saga **[V]**
`EnrollmentSaga::on_event` (`actors:415-419`) turns the handle stream's `learner-registered` into a `quiz.enroll-learner`
envelope (key `enroll:<catalog>:<handle actor id>`, principal `service:proctor`) for the learner stream; `unrelayed()`
(`:466-477`) finds the ones a crash lost; `Settler::{relay,drain,reconcile}` (`instance:361-461`). The projector is
*not* a saga (checkpointed fold, `projections` header). No other saga, timer or scheduled job exists; the only periodic
work is the 500 ms supervisor that settles sagas and projections (`SETTLE_INTERVAL` `instance:97`, `serve` `:594-605`).

---

## 2. Run lifecycle on the server

| Step | What happens | Where |
|---|---|---|
| Who picks the seed | The **client** picks the run id (`newId()`); `seed = fnv1a32(run id)`; the server only stores `seed` in `run-started` and in `RunState.seed`. The deputy uses `view.sheet.seed ?? runSeed(run)` (`deputy:149`), so the event's seed would be authoritative if the server ever chose it | `lifecycle:191`, `randomness:22` |
| Sheet assembly | `sheet_of(quiz, seed)` (pure): MT19937 from the seed; task order = shuffle with task 0 pinned first; per task in definition order: draw/shuffle items, categories (classification) or cards per dimension (matching); sorting never already ascending (rotate left). Called by the decider in `record_answer` and `submit_run` (recomputed every time, not stored) and by the projector for `RunView` | `Q/🔨️modules/🃏️sheet/🦀️.rs:15-31` |
| What the client gets | Start answer = events only. Then `quiz.run {run}` → stored `RunView` incl. the solution-free `Sheet { quiz, seed, title, description, tasks }` (session `loadRun`, `…/🧭️session/🟦️.ts:818-`). A **revised quiz** changes the recomputed `sheet` of old runs, `RunView` carries no revision | `views:63-77` |
| Answers | `record-answer` per task (coalesced in the client outbox). Validated only against the *sheet* of the run (`answer_rejection`), partial classification/matching allowed; stored in `RunState.answers` (latest wins), counts toward `answers_per_run` | `lifecycle:211-223`, `validation:189-209` |
| Submit | Requires revision unchanged (else the run is *voided*, not rejected), complete answers, then `score_run(quiz, sheet, answers)` = mean of task scores in sheet order (sorting: magnitude-weighted pair concordance; matching: same per dimension; classification: profile-similarity credit). `earned_badges` over *all* submitted results incl. the new one. Events `run-submitted`, `badge-awarded…` | `lifecycle:225-243`, `Q/🔨️modules/📏️scoring/🦀️.rs:40-50`, `Q/🔨️modules/🏅️badges/🦀️.rs:14-32` |
| Server wall clock | (a) HLC stamp per command at the lane (`S/…/gateway:2054`, `ServerState::now` `:1923`) → all `at`, `startedAt`, `submittedAt`; (b) `wall_clock()` (`queries:59`) is the instant a leaderboard period window is computed around, injected as `QuizQuery.now` (`instance:498`); (c) `Instant` for throttles and presence (not domain) | as cited |
| Leaderboard period | `period_window(period, at)` = UTC day / ISO week from Monday / month; a run counts when `window.from <= run.at < window.until` where `run.at` = `submittedAt` | `views:144-156,173-175` |
| Deadlines / durations | **none** (see fact 5). No `expires`, no timer, no "task opened" fact | grep result |
| Idempotency | command id = idempotency key; a replay answers the stored receipt and emits nothing; keys are per principal+target | README 237-239 |
| Decision purity | `decide_learner` receives only `(state, command, LearnerContext{now,catalog,quizzes,limits})`; `now` is the sole clock input, so time rules can be decided deterministically and tested with fixed `now` values (unit helper `context(millis)` `actors/tests:76`, `submit(bus, cmd, millis)` `projections/tests:21`) | `lifecycle:138-163` |

Client facts that interact (for the level design) **[V]**: `startRun` resumes an existing open run when the proctor
answers `run-open` (`session:804-807`); answers are applied locally first and delivered by the outbox
(`session:828-834`); `submit` waits until all answers are delivered (`session:848-863`); the session clock is
`options.now ?? Date.now` (`session:621`) and the deputy decides with it while offline.

---

## 3. Catalog, solutions, trust

**Server side [V].** `PROCTOR_CATALOG` = the catalog `🔣️.json`; `load_catalog(path)` reads it, resolves every quiz path
relative to it, parses each quiz, computes `revision = sha256_hex(quiz file bytes)` and a catalog fingerprint
(`🔨️modules/📚️catalog/🦀️.rs:94-128`), validates everything (`quiz::catalog_issues`, `quiz_issues`) and refuses the
whole boot on any issue. `LoadedCatalog { source, catalog, entries, current: BTreeMap<Slug, LoadedQuiz{quiz,revision}>,
view: CatalogView, fingerprint }`. The deciders hold `Arc<LoadedCatalog>` and decide with the full quizzes
(`LearnerDecider.catalog`). The catalog is baked into the image (README 70); a changed quiz changes its revision and
the fingerprint, which rebuilds the read models and voids/blocks open runs of the old revision. Dev catalog:
`🎓️teaching/🏛️architecture/❓️quiz/🔣️.json` (quizzes physics, heating, cooling, demand).

**What protects solutions from the client today [V].**
- `quiz.catalog` is solution-free (`catalog_view`, `views:34-45`); a sheet carries no item values; an open `RunView`
  has no `result`; `LearnerView` carries only scores. `submit` is the only moment solutions leave the server
  (`RunSubmitted.result` holds `value`, `correct`, `explanation`; visible in `RunView.result` and in the learner's own
  event stream).
- Hence the "sheet has no `value`" assertions (`e2e:388,421`).

**What does *not* protect them [V].** The site ships the same quiz documents with solutions
(`ARCHITECTURE_QUIZ_MATERIAL`), by design, for offline deciding. Also: the e2e/oracle tests read the quiz files; the
crowd view reveals the aggregate answers; thinking-room drafts reveal peers' answers and values. The model is "fun
quizzes": the proctor guarantees *scores, ranks and badges are computed by the server from recorded answers*, never
confidentiality of solutions (`README.md:464-467`, `Q/README.md:527`). The client's learner id is a bearer credential
(design §13/§18), and a device's provisional decisions never count: the proctor decides each delivered command again
(`Q/README.md:514-517`).

Implication for levels **[I]**: anything "hidden" on harder levels (keys, hints) is enforceable against the *UI* only.
What is enforceable by the server: the level a run was started at (event), whether an answer arrived inside its time
window (if a server clock fact exists), and the points multiplier. If tamper-resistance of hidden values is wanted, the
material bundle would have to stop shipping solutions (which also removes offline deciding) — a product decision, not a
proctor change.

---

## 4. Leaderboards, badges, crowd

### 4.1 Contribution of a run **[V]**
- `Transcript` (`views:106-114`) per ranked learner (has an identity and ≥ 1 submitted run): `runs: TranscriptRun
  { quiz, score, at }` (submitted runs, ordered by `at`, `submissions()` `:403-407`) and `badges: TranscriptBadge
  { badge, quiz (of the run that earned it), at }`.
- `standing(transcript, catalog, scope)` (`:225-231`): runs filtered by `BoardScope { window?, quiz? }`
  (`counts(quiz, at)` `:173`), `bests(&runs)` (`:410-420`) = max score per quiz (strictly greater replaces; `reached_at`
  = `at` of the run that last raised any best), `total(best, catalog)` = Σ over catalog quizzes in catalog order of
  `best × 100` (`:422-424`), `badges` = those whose run is in scope, `runs` = count in scope, `last_activity` = last run
  `at`. Returns `None` if no run in scope (the learner is not on that board).
- Order `Merit { total, badges (count), reached_at }::compare` (`:181-193`): total desc, badge count desc, reachedAt
  asc, learner id asc; rank = 1-based position.
- **Best run per quiz, summed over quizzes; not a sum of runs.** `LearnerView.best`/`total` use the same `bests`/`total`
  (`views:49-59`), so the home/profile totals follow the same rule.
- `Leaderboard` (`schema:877-890`): `rows` top 100 (`LEADERBOARD_TOP`), `learners` = count on that board, `submissions`
  = runs submitted in the whole catalog regardless of board (the client's trigger to refresh crowds), `own` the caller's
  true-rank row (caller = unauthenticated `learner` in the query).
- `LeaderboardRow { rank, tag, identity, total, reachedAt, best: {quiz→Score}, badges, runs, lastActivity }`
  (`schema:832-842`); `tag` = FNV-1a of learner id (hex8), never the id.

### 4.2 The proctor's `Board` **[V]** (`🔭️projections:120-276`)
In-memory `HashMap<learner, Arc<Transcript>>` plus `rankings: HashMap<(LeaderboardPeriod, Option<quiz>), Ranking
{ scope, order: Vec<Ranked{merit, transcript}> }>`; an index is built on the first query and rebuilt when the query
instant lies in another window; `set(transcript)` moves one entry per index (`Ranking::replace`). The answer rows are
made from the transcript through `quiz::standing`, so **any change in `standing`/`bests`/`total`/`Merit` is picked up
by the proctor automatically**; what the proctor adds is the index key `(period, quiz)` and the stored transcript
format. `Merit` holds exactly what orders a learner (`total`, `badges`, `reached_at`): a new ordering input (e.g. points
rather than score) must be inside `Merit`.

### 4.3 Badges **[V]**
`earned_badges(badges, quizzes, results: &[RunResult], held)` (`Q/🔨️modules/🏅️badges/🦀️.rs`): `perfect-quiz{quiz}` (some
result of the quiz has score `== 1.0`), `perfect-tasks{taskKind?,quiz?}` (every selected task has score 1 in some result),
`completed-quizzes`. Called in `submit_run` with every `RunResult` of the learner (`lifecycle:236-239`). `RunResult
{ quiz, score, tasks }` (`schema:542-546`) has no level and no run metadata. Level-aware badges (e.g. "perfect on
expert") need the level either on `RunResult` or a richer input than `&[RunResult]`; `BadgeRule` is
`deny_unknown_fields` in schema JSON, TS, Rust. A badge's contribution to a board is only its count (merit tiebreak).

### 4.4 Crowd **[V]**
`CrowdTally` per quiz (`🔨️modules/👪️crowd/🦀️.rs`), folded in the projector at `run-submitted`
(`projections:430-432`); `CrowdView` is `tally.view(&quiz)`; it must equal `quiz::crowd_view(quiz, results)` byte for
byte (e2e `:~700`, crowd unit test `:108-141`). `crowd_view` filters `result.quiz == quiz.id` (`views:306`). Making it
level-aware means: `fold(result, level)` keyed `quiz` or `quiz+level` (`TALLIES`/`CROWDS` keys, `reset` writing empty
views, `Query::Crowd { quiz }` + wire), `crowd_view(quiz, results)` gaining a filter, sorting `places` still binned
against `presented(task)`. Alternatively keep one crowd per quiz and mix levels (simplest but easy-with-keys runs
pollute the "what others answered" figures). **At fold time the projector already has the run's `RunState`** (it
applied `evolve_learner` before the match, `projections:424`), so a level stored in `RunState` is available to the crowd
fold even if `RunSubmitted` does not carry it.

---

## 5. Storage versioning and what a payload change needs

**Format row [V].** `proctor_format(singleton, schema, version)`; `FORMAT_SCHEMA = "semio.teaching.proctor.sqlite"`,
`FORMAT_VERSION = 2` (`🔨️modules/🗄️storage/🦀️.rs:62-66`). `Database::open` stamps a fresh file and **refuses** any other
(`check_format` `:425-430`: "holds format … v1; this proctor reads … v2"), also `backup`/`restore` verification. v2 =
one stream per handle, no roster, no `learner-recalled`. No migration (README 476-483).

**Dev launcher [V].** `settleDevelopmentData` (`🏗️bootstrap/🟦️.ts:133-150`) reads the format row read-only
(`storedProctorFormat` `:108-117`) of `.🧬semio/🎓️teaching/proctor-dev/` and, only if `schema` or `version` differ from
`PROCTOR_STORAGE_FORMAT = {schema, version: 2}` (`:70`), renames the folder to `proctor-dev.v<version>-<time>` and
starts empty, printing one line. A `PROCTOR_DATA`-named folder is never touched (error, exit 1). `proctor serve` never
moves anything. Nothing else (projector revision, state format, catalog fingerprint) triggers the move.

**What each change requires [V for the mechanisms, I for the recommendation]:**

| Change | Mechanism | Do |
|---|---|---|
| New event/command/state fields, all `#[serde(default)]` | old events still decode (types are `deny_unknown_fields` but missing optional fields are fine) | still bump the two below, because derived data is stale |
| Fold changes in `evolve_learner`/`RunState` | actor snapshots (every 64 events) are keyed by `state_format()`; other revision ⇒ ignored and replayed | **bump `STATE_FORMAT`** (`actors:79`, tested `actors/tests:187,240`) |
| Changed read models (`RunView`, `LearnerView`, `Transcript`, `CrowdView`, tally) | stamp `"{PROJECTOR_REVISION}:{fingerprint}"` | **bump `PROJECTOR_REVISION` 5→6** (`projections:89`); the boot rebuilds read models with progress; the test uses the constant (`projections/tests:280,287`) |
| New *required* fields, renamed fields, changed meaning of stored scores | old log would not decode, or would mean something else | **bump `FORMAT_VERSION` 2→3** in three places that must agree: `storage:66` (+ unit test `storage/tests:52` asserts `FORMAT_VERSION == 2 && …contains("v1;") && …contains("v2")`), `bootstrap/🟦️.ts:70`, README 476-483; any deployed volume/image data (not in production yet, deploy was prepared) would be refused. Dev data is then moved aside automatically |
| New command kind | manifest list (`instance:205-211`), `learner_template` action list (`:235`), `admitted`/`command_target`, TS `commandTarget`, instance unit tests pinning both lists (`instance/tests:15-16,29`) | |
| New rejection | `Rejection` enum + `as_str` (`schema:628-668`), JSON Schema `Rejection`, TS `REJECTIONS` (`schema/🟦️.ts:276`), `unknown` handling in the client | |
| New projection | name in `manifest().projections` (`instance:213`; pinned by `instance/tests:18`), `Projector::reset` clear list (`projections:344`) | |

The quiz data under dev is disposable (greenfield, no compat). Since the repo rule says "no legacy support", the clean
route is: bump `FORMAT_VERSION` (so dev data self-resets), `STATE_FORMAT` and `PROJECTOR_REVISION`, and make the new
fields required (no defaults) — then old files are refused or moved aside rather than silently reinterpreted. **[I]**

---

## 6. Tests, how they run, gates

Run: `bun nx run @teaching/proctor:test` → `📦️packages/🦀️rust/📜️script.ts test [fundamental|quick|long|exhaustive]` →
`runRepositoryCargoTests([PROCTOR_PACKAGE], <repo>/🎓️teaching, rest)` (`script.ts:41-46`); crate `teaching-proctor`
(Cargo workspace `🎓️teaching/Cargo.toml`, member `🛂️proctor/📦️packages/🦀️rust`, depends on
`semio-framework-quiz` by path). Test targets in `Cargo.toml`: lib unit tests (`#[path = "🧪️tests/🔬️unit/🦀️.rs"]` in each
module), `conformance` (`🧪️tests/🔬️conformance`, framework storage laws over the SQLite stores), `end_to_end`
(`🧪️tests/🌐️end-to-end/🦀️.rs`, 1315 lines, boots the real server on an ephemeral port and drives it with `ureq` and
`tungstenite`, restarts it on the same SQLite file). Launch rows: `🧪️test🎓️teaching🛂️proctor🦀️`, `📦️build…`, `⚖️gate…🏋️capacity`
(`.vscode/launch.json:4258,6729,6784`). The host note: big Windows cargo builds can exhaust memory; none was run here.

**Capacity gate** `bun nx run @teaching/proctor:capacity` (`🧪️tests/🏋️capacity/🟦️.ts`, 1101 lines): release build, 300
simulated learners from one address, two quizzes each at 5× pace, alone and beside an abusive script; fails on any
error, refusal inside the latency budget (p95 250 / p99 500 ms alone; 500 / 1500 beside), memory > 512 MiB, etc. It
builds `start-run` JSON itself (`:454`, `:678`: `{type:"start-run",id,learner,run,quiz}`) and the body limit is 16384
bytes with "largest real command 2952 bytes" (README 177); an *optional* level field breaks neither. A level that
slowed a command (e.g. per-command timing checks reading the log) would show up in this gate. It runs ~5 minutes; the
gate is not part of `test`.

**Tests that pin event/command/query shapes [V]:**

| Test | Pins | Path:line |
|---|---|---|
| `the_wire_names_follow_the_design` | `quiz.<type>` names, target kinds | `🔨️modules/🎭️actors/🧪️tests/🔬️unit/🦀️.rs:98` |
| `an_envelope_must_agree_with_its_command`, `no_malformed_id_and_no_refused_handle_is_admitted` | admission checks; build `Command::StartRun{…}` literals (`:142,146,148`) | `:112,133` |
| `the_caps_in_force_reach_the_learner_decisions`, `an_event_or_state_that_does_not_decode_poisons_the_actor_loudly` | caps, poison; `StartRun` literals (`:241,260,283`) | `:254,273` |
| `the_manifest_declares_the_quiz_surface` | the exact list of 5 commands, actor kinds, 6 queries with projections, 8 projections, 3 policies | `🔨️modules/🧩️instance/🧪️tests/🔬️unit/🦀️.rs:13-21` |
| `every_caller_may_learn_and_only_the_proctor_may_enroll` | policy grants per action | `:24-43` |
| projections: `played_runs_fold_into_learner_run_handle_and_leaderboard_views` | views, `LearnerView.total`, board == core `leaderboard` | `🔨️modules/🔭️projections/🧪️tests/🔬️unit/🦀️.rs:119-157` |
| `every_board_is_the_core_leaderboard_over_every_transcript…`, `a_board_asked_in_the_next_window…` | `Board` equals `quiz::leaderboard` at many instants; builds `TranscriptRun {quiz,score,at}` literals (`:86`) | `:348,386` |
| `only_a_submission_a_badge_or_a_registration_rewrites_a_transcript…` | which facts rewrite which projection | `:185` |
| `a_refold_and_a_restarted_projector…`, `projections_of_another_catalog_are_reset` (uses `PROJECTOR_REVISION - 1`) | rebuild and stamp | `:230,273-290` |
| `submitted_runs_fold_into_the_crowd_view_of_their_quiz`, crowd unit tests | tally == `crowd_view` bit for bit | `:404`, `🔨️modules/👪️crowd/🧪️tests/🔬️unit/🦀️.rs:108-165` |
| `each_read_answers_a_snapshot…`, `the_leaderboard_answers_the_top…` | query snapshots, builds `TranscriptRun` literals (`:44,46`) | `🔨️modules/❓️queries/🧪️tests/🔬️unit/🦀️.rs:52,73` |
| storage: `a_database_file_is_stamped_and_a_foreign_format_is_refused` | `FORMAT_VERSION == 2` and the message text | `🔨️modules/🗄️storage/🧪️tests/🔬️unit/🦀️.rs:39-53` |
| cli: `Command::StartRun` literal | | `🔨️modules/⌨️cli/🧪️tests/🔬️unit/🦀️.rs:435` |
| e2e `a_learner_plays_through_the_real_api_and_the_facts_survive_a_restart` | start-run event `run-started` with 64-char `revision`, `sheet.seed == events[0].seed`, **no `"value"` in the sheet**, 2 sheet tasks, badges order, board key set exactly `["learners","own","period","rows","submissions"]` (`:452`), learner `total`/`best`, period boards, restart equality | `🧪️tests/🌐️end-to-end/🦀️.rs:376-498` (play helper `:364-372`, reads solutions from the quiz file `:319-357`) |
| e2e crowd test | `crowd` bytes == `quiz::crowd_view` over the results, mirrored places | `:626-716` |
| e2e idempotency/edge | `start-run` literals for sign-up, id admission, body limit, allowance | `:999-1170` |

Quiz-core side (not under the proctor but pinned together with it): fixtures `Q/🧫️fixtures/🧾️learner-lifecycle` (9.8k lines),
`🏆️leaderboard` (16k), `🏅️badge-rules`, `📊️crowd-view`, `🧬️schema-conformance`, `🃏️sheet-assembly`; tests
`Q/🧪️tests/🧾️learner-lifecycle/{🦀️.rs,🟦️.ts,🐍️.py,🥒️.feature}`, `🏆️leaderboard/*`, `📊️crowd-view/*`, `🃏️sheet-assembly/*`,
`🔁️run-lifecycle/🟦️.ts`, `🫡️deputy-decisions/🟦️.tsx`, `📬️outbox-delivery/🟦️.tsx`; the draft-07 schema
`Q/🧬️schema/🔣️.json` (Command `:613`, start-run `:633`, Rejection `:666`, Event `:688`, run-started `:708`, RunView
`:823`, RunSummary `:844`, LearnerView `:869`, LeaderboardRow `:888`, Leaderboard `:925`, CrowdView `:1158`, Limits `:600`).

---

## 7. Extension points, file by file

### 7.1 Carry `challenge` on run start

| File | Change |
|---|---|
| `Q/🧬️schema/🔣️.json`, `🟦️.ts` (`StartRunCommand` `:264`), `🦀️.rs` | New enum (e.g. `Challenge = easy|medium|hard|expert`, kebab-case) on `Command::StartRun`, `Event::RunStarted` (so the fact carries it), `RunSummary`/`RunView`/`Sheet` as needed; a decode error of an unknown value already surfaces as `command-malformed: …` (`actors:207`) but is not in the client's `REJECTIONS`; add `challenge-invalid`/`level-unavailable` if the server may refuse a level (e.g. per quiz) |
| `Q/🔨️modules/🧾️lifecycle/🦀️.rs`, `🟦️.ts` | `start_run(state, run, quiz, challenge, ctx)` copies it into `RunStarted`; `RunState.challenge` (state bytes, snapshot) and `evolve_learner`; decide what an open run of another level means (`run-open` today ⇒ the client resumes the old level; either void/replace the open run on a level change or make the level part of the "same open run" test) |
| `Q/🔨️modules/🃏️sheet/🦀️.rs`, `🟦️.ts` | `sheet_of(quiz, seed, challenge)`; RNG consumption order must not change per level (it is "part of the contract", `sheet/🦀️.rs:3-5`) so seeds keep producing the same order across levels, keys/hints are added after the shuffle. Callers: `lifecycle` (record_answer, submit_run), `views::run_view`, `scoring::score_run` (takes the sheet), deputy `run()`/`runView`, `Q/🧪️tests/🃏️sheet-assembly`, fixtures `🃏️sheet-assembly` (4.5k lines), `🎴️sheet-randomization` |
| `Q/🔨️modules/👁️views/🦀️.rs`, `🟦️.ts` | `run_view`: pass `found.challenge` to `sheet_of`; `learner_view` summaries; `RunSummary.challenge` |
| `Q/🔨️modules/✅️validation/🦀️.rs`, `🟦️.ts` | `answer_rejection` is against the sheet, so unchanged unless a level forbids guesses (`SortingAnswer.guesses`) or hints |
| `🔨️modules/🎭️actors/🦀️.rs` | nothing structural: `admitted()` decodes `Command` generically (`:207`), `record()` serializes `Event` (`:406`); bump `STATE_FORMAT` (`:79`) |
| `🔨️modules/🔭️projections/🦀️.rs` | bump `PROJECTOR_REVISION`; `RunView` is regenerated from `RunState`, so no code change except through `quiz::run_view`; transcript/crowd see 7.3 |
| `🔨️modules/🗄️storage/🦀️.rs`, `🏗️bootstrap/🟦️.ts:70`, `README.md` | `FORMAT_VERSION` 3 (see 5) and the storage unit test `:52` |
| `Q/🎯️targets/⚛️react/🔨️modules/🧭️session/🟦️.ts:802`, `…/🛂️proctor/🟦️.ts`, `…/🫡️deputy/🟦️.ts` | client sends the level; deputy decides with it and uses it in `alike(view.sheet, sheetOf(quiz, seed, level))` (`deputy:151`); *not* the proctor's job but the proctor needs the same sheet function |
| Tests to update | `StartRun` and `TranscriptRun` literals listed in 6; e2e `:365,416,…` (optional field keeps them compiling only if it is `Option`/`#[serde(default)]`; if required, every literal changes); add e2e: start at each level, `RunView.sheet` per level, "no value in sheet" assertion becomes "no value on medium/hard/expert" |
| Docs | `README.md` (Wire 336-399), `Q/README.md`, design addendum |

### 7.2 Enforce per-question time limits on expert

What exists **[V]**: `now` in the decider; `RunState.started_at`; `AnswerRecorded.at`; no task-level fact; no scheduler.
What is missing and where it would live **[I]**:

| Need | Option (server-authoritative) | Files |
|---|---|---|
| A server fact for "this question became visible" | New command `quiz.open-task {id, learner, run, task}` → event `task-opened {learner, run, task, at}` (server `at`), idempotent per `(run, task)`; deadline = `at + limit(level, task)`. The client calls it when it shows the question (or the server could also consider the previous task's closing as opening) | schema (Command/Event/Rejection), `lifecycle` (`open_task`, `RunState.opened: BTreeMap<Slug, Timestamp>`, `record_answer` check `context.now > opened[task] + limit` ⇒ reject `task-timed-out` or accept as "late" and score 0), manifest `instance:205-211`, `learner_template` `:235`, `admitted`, TS `commandTarget` unchanged (target = learner), instance tests `:15-16,29`, README |
| Time rules without a new command | Use `started_at` and the cumulative budget (`n_tasks × limit`) checked at `record-answer` and `submit-run`: coarse (no per-question), but needs no client cooperation | `lifecycle` only |
| Late delivery (outbox, deputy replay) | A pure server rule on arrival `at` penalises slow networks. Fairness options: (a) grace window; (b) the client attests elapsed ms in the command (untrusted but bounded by `at − opened_at`: use `min`); (c) accept the answer, flag it `late` on the event and score by rule. The envelope `clientHlc` is unused by the authority, and a command field would have to be validated against `at` | decision for the coordinator |
| Expiry without a trigger | The proctor has no timer. Lazy evaluation at next `record-answer`/`submit-run`/`start-run` is deterministic and fits event sourcing. An *unsubmitted expired* run stays `open` unless `start-run` voids it (today: `run-open` ⇒ client resumes it). Add: in `start_run`, treat an open run whose time is over as voidable | `lifecycle:183-195` |
| What an expired question scores | `submit_run` requires `answer_complete` for every sheet task (`:232`); timed-out tasks need a defined "unanswered" completion (score 0 task result) or `run-incomplete` would trap the run. `score_run` is `None` without an answer per task (`scoring:40-50`) | `lifecycle`, `scoring`, results/crowd handling |
| Deputy parity | The deputy runs the same deciders with the device clock; it must either implement the same rule (its decisions are provisional and replaced by the proctor's) or expert must be `AuthorityRequired` (like `OfflinePolicy` of identify/start/submit, `instance:206-209`) | `…/🫡️deputy/🟦️.ts`, `session` |
| Test seam | Pure `now`: unit-test with `LearnerContext.now` and `context(millis)`; e2e cannot control the server clock (wall clock) so it must use limits of a few hundred ms, or a configurable limit via `Limits`/env (`PROCTOR_*` plumbing `config:233`, `Admission::cap`) |

### 7.3 Rank by level-weighted points

Decisions to make **[I]**: points of a run = `score × 100 × multiplier(level)`; per quiz take the max over runs of
*points* (so a 0.8 on expert can beat a 1.0 on easy), total = Σ over quizzes (unchanged shape); or max per (quiz, level)
summed (more points available, 4× the surface). The core functions to change:

| File | Change |
|---|---|
| `Q/🧬️schema/*` | `TranscriptRun { quiz, score, challenge, at }` (stored in `quiz.leaderboard`); `LeaderboardRow.best` is `{quiz→Score}` with `Score ∈ [0,1]`: either keep `best` as the raw best score of the best-pointed run and add `points`/`bestPoints`, or turn it into points (client leaderboard, `LearnerView.best/total`, profile, home all read it); `LearnerView.best`; multiplier table as a constant in the core (both languages) or in the catalog (data, `Catalog` is `deny_unknown_fields`) |
| `Q/🔨️modules/👁️views/🦀️.rs` | `submissions()` (`:403`) fills the level from `RunState`; `bests()` (`:410`) compares points; `total()` (`:422`) sums points; `Standing`, `Merit` (`total` already f64, keep `Merit` 24-32 bytes), `standing()`; `transcript()` (`:119`); `TranscriptBadge` unchanged. TS twin `Q/🔨️modules/👁️views/🟦️.ts:73-192` and Python reference `Q/🧪️tests/🏆️leaderboard/🐍️.py` + fixtures must change together |
| `🔨️modules/🔭️projections/🦀️.rs` | `Ranking`/`Board` need no code change (they call `standing`/`Merit`); `Transcript` format changes ⇒ `PROJECTOR_REVISION` bump; `submissions` counter (`fill`, `set`) unaffected |
| `🔨️modules/❓️queries/🦀️.rs` | none unless a level filter is added to `Query::Leaderboard` (`period, quiz?, learner?, challenge?`): then `Board` key `(period, quiz, challenge)` (`projections:132`), `BoardScope` gets `challenge`, `Ranking::of`; the e2e exact key-set assertion for the all-time board (`e2e:452`) must include any new field |
| Client | leaderboard page, profile, home card read `total`/`best`; out of scope here |
| Where `reachedAt` goes | stays the `at` of the run that last raised a best (points) |

Also **[V]**: the mean-of-tasks score, partial credit and pair-concordance scoring are level-independent and live in
`score_run`; per-level scoring variants (e.g. hints cost) would go in `score_run`/`submit_run` and into `RunResult`.

### 7.4 Other places a level touches

- `quiz.learner`/`RunSummary`: show the level of each run (`views:54`).
- Badges: `earned_badges` signature (see 4.3).
- Crowd: see 4.4; query `Crowd { quiz }` (`schema:929`), `QuizQuery` (`queries:97`), `CROWDS`/`TALLIES` keys, `reset`.
- Presence/thinking: `thinking_scope(catalog, quiz)` (`Q/🔨️modules/👥️presence`, `🔨️modules/👥️presence/🦀️.rs`) is per quiz;
  drafts of peers disclose answers; per-level rooms would be a presence-module change (`Rooms::of`).
- Hints: a server-computed hint is a new fact (`hint-given`) if it must be counted/limited; a static hint in the easy
  sheet needs nothing but the sheet function. The server never computes anything on demand today (queries only read
  projections, except `Board::view`).
- Caps: `Limits` (`schema:567-578`) has no level; answers per run 2000.
- Per-quiz availability of levels would belong to the quiz/catalog document (`Quiz`, `Catalog` are
  `deny_unknown_fields`; revision = SHA-256 of the file bytes ⇒ any quiz edit voids open runs).

---

## 8. Other observations worth knowing

- `quiz.run` has no ownership check; it returns `learner` (the bearer credential) together with the run. The run id is
  128 random bits, shared only by the owner; presence and leaderboards carry tags. **[V]**
- `RunView.sheet` is recomputed from the *current* quiz at fold time and stored; a revised quiz is only reflected after
  the next event of that run or a rebuild (the catalog fingerprint stamp triggers a rebuild at boot). **[V]**
- The deputy's staleness test `alike(view.sheet, sheetOf(loaded.quiz, seed))` (`deputy:151`) assumes the sheet is a pure
  function of `(quiz, seed)`. A level that changes the sheet must reach this function or every offline-held run of a
  non-default level is judged "revised" and voided. **[V]**
- `Deputy` revisions use `materialRevision(quiz)` = FNV-1a of the JSON, the proctor uses SHA-256 of file bytes; they are
  never compared across (deputy header). **[V]**
- Timing of `submittedAt` relative to leaderboard windows: a run started Monday 23:59 and submitted Tuesday counts on
  Tuesday's daily board (`TranscriptRun.at = submitted_at`). **[V]**
- `command_allowance` is the framework hook that already meters per-client allowances; only `identify-learner` uses it
  (`instance:193-195`). A cost-bearing expert action would use the same hook if needed. **[V]**
- The sign-up rate, 16 KiB body and per-address read limits (README 164-183) are unaffected by a one-word field. **[I]**
