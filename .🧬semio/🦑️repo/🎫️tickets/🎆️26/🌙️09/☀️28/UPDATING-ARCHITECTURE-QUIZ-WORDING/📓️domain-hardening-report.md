# 📓️ Domain hardening of the quiz core and the proctor

Agent "domain", 2026-10-02. Scope: the quiz product cores (TypeScript and Rust twins), their schema and conformance
cases, and in the proctor the actors, projections, queries, command line, the non-receipt parts of storage and the
README. Everything below was run on this host; commands and their results are quoted in [Gates](#gates).

**Outcome in one paragraph.** Ids, slugs and handles are held to their shapes wherever they enter. Handles are
normalized by the server under one documented alphabet, identically in both cores. The roster actor is gone: every
handle is its own one-event stream, a recall is a read that writes nothing, and every state is capped. The leaderboard
is `{ rows ≤ 100, learners, own? }`, kept as a rank index that moves one entry per submission and is never touched by
an answer. The proctor has `health`, `backup`, `restore` and `erase`. The real site catalog is played in the
conformance suite. An event the proctor cannot decode stops the fold loudly.

## 1. Ids and slugs

| | |
|---|---|
| **Decision** | An id is exactly `^[0-9a-f]{32}$`, a slug `^[a-z0-9]+(?:-[a-z0-9]+)*$` with at most 64 characters. One validator per core holds every id of a command or query to its shape; the rejection code `id-invalid` is in the schema. It runs at every door: the pre-placement hook, the deciders, the queries. |
| **Change** | Schema: `Rejection` gains `id-invalid`. Cores: `isId`, `isSlug`, `commandRejection(command)`, `queryRejection(query)` (`is_id`, `is_slug`, `command_rejection`, `query_rejection`); `decideHandle`/`decideLearner` call `commandRejection` first. Proctor: `actors::Admission::admit` (edge's `ServerModule::command_admission` hook calls it) refuses a target that is no quiz actor's shape (`actors::addressable`) before the payload is decoded, then holds the decoded command to its shapes, its envelope and its own target; the deciders ask the same `Admission` again; `QuizQuery` answers `400 id-invalid: …` before any lookup. |
| **Tests** | Protocol v2 case `🪪️identity-shapes`, scenario `shapes` (42 commands and queries, both cores against python-jsonschema over the schema). Unit: `actors::tests::no_malformed_id_and_no_refused_handle_is_admitted` (megabyte ids, upper case, trailing line feed, path fragments, foreign actor kinds, a target refused before an undecodable payload is read), `queries::tests::no_malformed_id_is_looked_up` (rows planted under the malformed keys are not found), `instance::tests::the_admission_the_bus_asks_is_the_one_the_deciders_hold_and_carries_the_caps`; end to end: edge's `a_command_for_an_id_no_actor_can_have_is_refused_before_it_costs_anything` passes unchanged, and the lifecycle test asks `quiz.learner` for `../../etc/passwd` (400). |

## 2. Handles (S1)

| | |
|---|---|
| **Decision** | The server normalizes; the client is never trusted. Alphabet: digits, 681 Latin letters (Basic Latin, Latin-1 Supplement, Latin Extended-A/-B, Latin Extended Additional — umlauts, `ß`, accents), `'` `.` `_` `-`, single spaces between words; `’` becomes `'`; every `White_Space` run is one space; 1…64 code points with at least one letter or digit; at most 256 typed. Everything else is `handle-invalid`: control, format, zero-width and bidirectional characters, combining marks, other scripts, emoji. **NFC without a library:** the alphabet has no combining mark and only NFC-stable letters, so admitted handles are NFC and canonically equivalent handles are equal code points; a decomposed spelling is refused, not composed. The tables are owned code (no runtime dependency). |
| **Change** | Schema `$defs/Handle` (pattern = the alphabet), `Identity.handle → Handle`, `IdentityClaim` for what a client types. Cores: `normalizeHandle` → `{ display, key }`, `HANDLE_MAX`, `HANDLE_INPUT_MAX`, `WHITE_SPACE`, `HANDLE_LETTERS`, `HANDLE_PUNCTUATION`, `handleActorId(key)`, `handleKeyOf(actor)`. Presence states carry only normalized handles (`handle-invalid`). **Recall is a read:** the event `learner-recalled` no longer exists; `Query` gains `{ type: "handle", handle }` → `HandleView { display, holder? }`; the proctor answers it from the projection `quiz.handle`. |
| **Tests** | `🪪️identity-shapes` scenarios `handles` (74 vectors: NFC/NFD pairs, zero-width, bidi overrides, controls, homoglyphs, overlong, apostrophes) and `alphabet` (every Unicode scalar classified and 340 lowercase folds, so a Unicode version skew between Python, ICU and Rust would show). Oracles: python-jsonschema over the schema pattern held to `unicodedata`; Rust unit tests hold the tables to the `unicode-normalization` crate (dev-dependency) and `char` properties; TS unit tests to ICU. Ticket script `domain_handle_alphabet.py` derives the alphabet from the UCD: `681 letters … pairs that compose: 0 of 484416`. Proctor: `queries::tests::a_handle_is_recalled_by_a_read_that_writes_nothing` (log head stays 0), e2e lifecycle test (recall typed `" ADA  lovelace"`, the learner stream has one event before and after, five refused handles), live run through nx with `Zoë O'Neill-Müller`. |

## 3. Bounded growth (S2)

| | |
|---|---|
| **Decision** | No roster. A handle key is the stream `quiz-handle/<lowercase hex of the key's UTF-8 bytes>` with exactly one event; claiming is exactly-once by that actor's turn; who holds a handle is one indexed read of a projection derived from the events. Caps in the core (`Limits`, `DEFAULT_LIMITS`): 10 000 registrations (`roster-full`), 1 000 submitted runs per learner (`runs-exhausted`), 200 of them per quiz (`runs-exhausted`), one open run per learner and quiz (`run-open`), 2 000 recorded answers per run (`answers-exhausted`). The two totals are configured by `PROCTOR_MAX_LEARNERS` and `PROCTOR_MAX_RUNS`. |
| **Change** | Cores: `HandleState`, `decideHandle`, `evolveHandle`, `registrationRejection(learners, limits)`, `LearnerContext.limits`, `RunState.recorded`. Proctor: `HandleDecider`, `LearnerDecider`, `Admission` (caps as atomics, learner count from the projector's gauge), `EnrollmentSaga` over handle streams with the key `enroll:<catalog>:<handle actor id>` (names no learner), `unrelayed` reading only handle streams; projection `quiz.handle`; `ProctorConfig::caps`, `Proctor::capped`. Storage format **v2** (`FORMAT_VERSION`). The count the cap is held against is one per anonymous learner **and one per claimed handle**, so a client that claims many handles under one learner id cannot grow the handle streams past the cap either. |
| **Tests** | `🧾️learner-lifecycle` scenarios `registrations` (claims, second claims, quotas) and `learner-decisions` (sequence `caps`); `actors::tests::a_full_proctor_admits_no_registration_but_every_other_command`, `…the_caps_in_force_reach_the_learner_decisions`, `…a_handle_registers_its_first_claim_and_refuses_the_rest…`, `…enrollment_relays_a_handle_registration_to_its_learner_once`; `projections::tests::every_claimed_handle_counts_as_a_registration_even_when_one_learner_claims_them_all` (also: a replayed claim counts once); `config::tests::the_quiz_caps_default_to_the_cores_and_the_two_totals_are_configurable`; e2e: `handle-claimed` before and after a restart. |
| **Documented** | Proctor README "Identity: handles, ids and caps"; quiz README "Run lifecycle" and "Identity". |

## 4. Leaderboard

| | |
|---|---|
| **Decision** | Exactly `Leaderboard { rows: top ≤ 100; learners: integer; own?: LeaderboardRow }`. Ranking unchanged (total ↓, badge count ↓, `reachedAt` ↑, learner id ↑). The caller is named in the query (`{ type: "leaderboard", learner? }`) because the proctor resolves no principal. Rows carry the public tag, never a learner id. |
| **Change** | Cores: `Standing`, `standing(state, catalog)`, `compareStandings`, `leaderboard(standings, caller?)`, `LEADERBOARD_TOP`. Proctor: the projection `quiz.leaderboard` holds one standing per learner; `projections::Board` keeps them in rank order in memory (loaded once from those rows); the projector rewrites a standing only on `learner-registered`, `run-submitted` and `badge-awarded` and only when it changed; an answer rewrites the learner's folded state and its run view and nothing else. A query takes the top slice, the length and one binary search for `own`. `LeaderboardRow.lastActivity` is now the latest **submission** (it used to move with every event). |
| **Tests** | `🏆️leaderboard` scenarios `rankings` (per caller) and `cuts` (130 standings: top 100, count, own beyond the top), both cores. `projections::tests::the_board_is_the_core_leaderboard_over_every_standing_whatever_order_they_arrive_in` (333 standings, updates, four callers, equality with `quiz::leaderboard`), `…only_a_submission_a_badge_or_a_registration_rewrites_a_standing…` (bytes of both standings, the learner view and the board unchanged across answers), `…played_runs_fold_into_learner_run_handle_and_leaderboard_views` (no learner id in the board), `queries::tests::the_leaderboard_answers_the_top_the_count_and_the_callers_own_row_from_the_board` (a loaded board answers after the standings table is cleared); e2e: exact key set `learners, own, rows`, an unranked caller, same answer after a restart. |

## 5. Operator verbs

Final syntax (also in the proctor README, "Operate: health, backup, restore, erase", with the Docker one-off
invocations):

```text
proctor health
proctor backup <directory/|file|->
proctor restore <file|->
proctor erase (--handle <handle>|--tag <tag>|--learner <id>) [--dry-run]
```

| Verb | Decision and change | Tests |
|---|---|---|
| `health` | Deploy's `ready` renamed; `ready` no longer exists. Exec form, no arguments: `GET /instance` as the proxy asks it, exit 0 on `200`. | `cli::tests::health_asks_the_listener_as_the_proxy_would`; e2e with the built binary (serving → 0, stopped → 1); `bun nx run @teaching/proctor:health` live: exit 0, after the stop exit 1. |
| `backup` | Deploy's `VACUUM INTO` copy kept and reshaped (`Database::snapshot(directory, target, cancel, progress)`): the name is reserved with `create_new` (never overwritten), the copy is written as `<name>.partial-<pid>`, verified (`Database::inspect`: format row, integrity check; its event count must lie between the source's before and after) and renamed; progress every 100 ms, cancellation interrupts SQLite, removes both files and exits 130. A directory (existing, or a path ending in a separator) gets `proctor-<UTC time>.sqlite`; `-` streams through a spool in `PROCTOR_DATA` (docker-cp-free, read-only root). Prints the path. | `storage::tests::a_snapshot_taken_while_serving…` (progress reaches the whole, nothing partial left, an existing target byte-identical after the refusal, a cancelled copy leaves nothing); `cli::tests::a_backup_restores_into_another_data_directory`, `…a_backup_into_a_directory_is_named_after_its_time…`, `…a_backup_is_named_by_the_utc_second_it_was_taken` (six known instants incl. leap days); e2e with the binary while serving. |
| `restore` | Deploy's verb kept: spool, verify, adopt; refuses while any process holds the database. The "in use" probe now waits 250 ms instead of SQLite's 5 s default. | cli tests above (garbage refused, no spool left, database untouched); **e2e: a proctor is booted on the restored copy — from stdin and from the file — and answers the same learner, run, leaderboard, crowd, handle and catalog views**, then appends. |
| `erase` | New. Offline only (`Database::open_offline`). Selects one learner by handle (normalized), public tag or id, read from the streams themselves. In one transaction removes the learner's stream, the stream of every handle it holds, their receipts, outbox rows and deliveries, snapshots and leases, and every read model; then `secure_delete` + `VACUUM` + WAL truncation. `--dry-run` reports and changes nothing. **Scores: removed with the name** — the learner id is still held by the person, so anonymized scores would stay personal data; the README says so. The handle is free again. | `storage::tests::an_erasure_removes_every_trace_of_its_actors_from_the_file_and_nothing_of_anybody_else` and `cli::tests::erase_removes_one_learner_from_a_stopped_proctor_and_frees_its_handle` search every file of the data directory for the bytes of the name, its key and the learner id; `…a_selector_names_the_learner_and_every_handle_it_holds`, `…erase_takes_exactly_one_selector…`; e2e `the_operator_erases_one_learner_on_request_and_its_handle_is_free_again` (refused while serving, dry run, erase, 404s, board and crowd without the learner, the other learner's views byte-equal, the handle registered by somebody else). Live through nx. |

Registration: nx targets `health`, `backup`, `restore`, `erase` in `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json`
(each `bun ./📜️script.ts <verb>`), the verbs in that package's `📜️script.ts`, and five launch rows in **both**
`.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` after `🔁️rebuild🎓️teaching🛂️proctor`:
`🩺️health…`, `💾️backup…`, `♻️restore…`, `🧨️erase…🔍️dry-run`, `🧨️erase…` (inputs `proctorBackupFile`,
`proctorEraseHandle`).

## 6. Real-catalog badges

| | |
|---|---|
| **Decision** | The conformance suite plays the catalog the site serves, read from the repository at run time (`🎓️teaching/🏛️architecture/❓️quiz/🔣️.json` and its four energy quizzes), in both cores against the Python reference. |
| **Change** | `🧾️learner-lifecycle` scenario `site-catalog`: 12 plays — perfect tours earn every one of the 7 badges, named checks for "Heating expert", "Numerical Brain" and "Pattern seer"; nine tours with a single mistake withhold exactly the badges that depend on the flawed task; every badge is earned by some play, so every badge is reachable. |
| **Tests** | Part of parity (105/105) and of the TS shared-vector suite. |

## 7. Undecodable events

| | |
|---|---|
| **Decision** | Nothing is skipped silently. The fold stops with an error naming position, stream, sequence, kind and cause; the checkpoint stays before the event; the proctor does not listen and `rebuild` exits non-zero. An actor whose stream or state does not decode is poisoned: every command answers `actorUnavailable` with the cause, one `[ERROR]` line is logged, no other learner is affected. |
| **Change** | `Projector::catch_up` is strict (unknown stream kind, undecodable payload, a fact of another learner, a registration in the wrong handle stream, an undecodable stored state, tally or standing); `projections::decoded`; `actors::Stored`/`Poison`/`evolved`. |
| **Tests** | `projections::tests::an_event_the_fold_cannot_read_stops_it_with_its_position_and_cause` (a `learner-recalled` event from the old vocabulary: the message, checkpoint 0, the batch not written, the same failure on the next catch-up), `…a_stream_the_proctor_does_not_know_and_a_misplaced_fact_stop_the_fold` (five causes), `…a_stored_state_or_standing_that_does_not_decode_is_an_error_not_a_fresh_start`, `cli::tests::rebuild_fails_over_a_log_it_cannot_read` (exit code), `actors::tests::an_event_or_state_that_does_not_decode_poisons_the_actor_loudly`. |

## Contract changes the UI has to follow

Written for the UI agent in `🗒️domain-notes-for-ui.md`; in short:

- `Leaderboard` is `{ rows, learners, own? }`; the query carries the caller: `{ type: "leaderboard", learner? }`.
- A named `identify-learner` targets `quiz-handle/<handleActorId(normalizeHandle(handle).key)>`; an anonymous one
  `quiz-learner/<learner>`. Recall is the query `{ type: "handle", handle }`; `learner-recalled` is gone.
- New rejections: `id-invalid`, `handle-claimed`, `learner-exists`, `roster-full`, `runs-exhausted`,
  `answers-exhausted`; a refused query is HTTP 400 whose message starts with `id-invalid` or `handle-invalid`.
- `LeaderboardRow.lastActivity` is the last submission. Handles in events, views and presence are always normalized.
- I made no edit in `🎯️targets/⚛️react`.

## Served: what edge asked of me (`🗒️edge-needs-from-domain.md`)

- `instance::admissible`, `ID_INVALID`, `KEY_BYTES` and their unit test are deleted; the hook is `self.admission.admit`.
- `admit` is cheap and store-free (envelope fields, one decode, two atomic reads) and bounds every id and key.
- `actors::enrollment(&EventRecord) -> Option<CommandEnvelope>` is still public and a function of the committed event;
  edge's `an_anonymous_caller_can_neither_read_nor_occupy_an_enrollment_key` passes (the key names the handle stream,
  not the learner).
- `Settler::reconcile` reads through the log only (`actors::unrelayed`), the bus only for the `submit`.
- Both deciders implement `state_format()` (`actors::STATE_FORMAT`).
- Status and the final verbs are in `🗒️domain-needs-from-edge.md`.

## Gates

Run on 2026-10-02 after the last change, on this Windows host.

| Command | Result |
|---|---|
| `RUSTC_WRAPPER="" bun nx run-many -t test -p @semio-tech/quiz @semio-tech/quiz-rs @teaching/proctor --skip-nx-cache` | exit 0 — `Test Files 10 passed (10)`, `Tests 273 passed (273)`; quiz-rs `94 passed … 2 filtered out` (the two are `quick::` tests); proctor `76 passed` (unit), `15 passed` (conformance), `14 passed` (end to end); `Successfully ran target test for 3 projects` |
| `cd "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test" && SEMIO_TEST_BUDGET_MS=900000 RUSTC_WRAPPER="" bun ./📜️script.ts parity exhaustive --owner "🧰️framework/🛍️products/❓️quiz"` | `[test] level=exhaustive cases=13 executed=105 passed=105 failed=0 errored=0 parity=105/105` (was 93/93) |
| `bun ./📜️script.ts verify taxonomy report --scope "🧰️framework/🛍️products/❓️quiz"` | `clean=true errors=0 warnings=0` |
| `bun ./📜️script.ts verify taxonomy report --scope "🎓️teaching"` | **`clean=false errors=1`** — `directory-kind-unresolved 🎓️teaching/🛂️proctor/🧪️tests/🏋️capacity`: edge's new test directory is not registered in the taxonomy. Not mine; reported to edge in `🗒️domain-needs-from-edge.md`. Nothing of mine is flagged. |
| `RUSTC_WRAPPER="" cargo clippy -p semio-framework-quiz --all-targets --features sut -- -D warnings` | exit 0 |
| `RUSTC_WRAPPER="" cargo clippy -p teaching-proctor --all-targets --no-deps -- -D warnings` | exit 0. Without `--no-deps` the same command fails inside workspace dependencies I do not own (`semio-framework-dispatch-macros`, `semio-framework-trace`, the value derive). |
| Live, through nx, on a private port and data directory | `@teaching/proctor:dev` served; `:health` exit 0; two registrations and a recall over HTTP; `:backup -- <dir>/` printed `…/proctor-20261001T235103Z.sqlite` and `[INFO] backup: 90112 bytes, 4 events up to position 4, verified`; `:erase` while serving exit 1 (`… is in use …`); after the stop `:health` exit 1, `:erase -- --handle "ada  LOVELACE" --dry-run` reported, `:erase -- --handle="Zoë O'Neill-Müller"` erased, the same again exit 1 (`nobody … matches`). |

## Open issues and things to know

1. **Taxonomy, `🎓️teaching` scope:** one error from edge's unregistered `🧪️tests/🏋️capacity` (see above).
2. **Development data is discarded.** Storage format v2 refuses a v1 file; there is no migration (nothing was in
   production). Delete `.🧬semio/🎓️teaching/proctor-dev/` (and the e2e scratch directories) once.
3. **The registration cap is a quota, not a ledger.** It is held against the projector's count, which trails the log by
   the commands in flight; a burst can pass it by the registrations decided before the fold caught up.
4. **One registration per learner id.** A handle actor cannot see whether its claimant is already registered: a
   learner id that claims a second handle holds both handles and keeps its first identity. It is counted against the
   cap and documented; refusing it would need a second, compensating saga.
5. **Per-command cost** is one learner's state (decode, decide, encode), bounded by the caps; there is no compaction
   of old runs.
6. **Old backups keep erased names.** `erase` rewrites the live file only; the README tells the operator to delete or
   re-erase backups.
7. **The ticket's `📓️design.md`** still describes the roster actor and `learner-recalled` (§8, §9a). This report and
   the two READMEs supersede those parts; I did not rewrite the design document.
8. **`cargo-nextest` is not installed on this host**, so the test runner falls back to sequential `cargo test`; the
   proctor's fundamental suite takes about 6 s of its 15 s budget after the "in use" probe stopped waiting 5 s.
9. **The React target and the capacity gate** (which drives the React `ProctorClient`) must follow the contract
   changes; that is the UI agent's and edge's work.

## Files

**Created**

- `🧰️framework/🛍️products/❓️quiz/🧪️tests/🪪️identity-shapes/` — `🥒️.feature`, `🐍️.py`, `🟦️.ts`, `🦀️.rs`
- `🧰️framework/🛍️products/❓️quiz/🧫️fixtures/🪪️identity-shapes/🔣️.json`
- Ticket: `📓️domain-hardening-report.md`, `🗒️domain-notes-for-ui.md`, `🗒️domain-needs-from-edge.md`,
  `domain_handle_alphabet.py`, `domain_vector_summary.py`, `domain_operator_probe.py`, `domain_docstring_emojis.py`

**Updated — quiz product**

- `🧬️schema/🔣️.json`, `🧬️schema/🟦️.ts`, `🧬️schema/🦀️.rs`, `🧬️schema/🧪️tests/🔬️unit/🦀️.rs`
- `🔨️modules/✅️validation/{🟦️.ts,🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`
- `🔨️modules/🧾️lifecycle/{🟦️.ts,🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`
- `🔨️modules/👁️views/{🟦️.ts,🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`
- `🔨️modules/👥️presence/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`
- `📦️packages/🦀️rust/Cargo.toml` (dev-dependency `unicode-normalization`)
- `🧪️tests/🧾️learner-lifecycle/{🥒️.feature,🐍️.py,🟦️.ts,🦀️.rs}`, `🧪️tests/🏆️leaderboard/{🥒️.feature,🐍️.py,🟦️.ts,🦀️.rs}`,
  `🧪️tests/🧬️schema-conformance/🥒️.feature`
- `🧪️tests/🔁️run-lifecycle/🟦️.ts`, `🧪️tests/👁️read-views/🟦️.ts`, `🧪️tests/🗃️shared-vectors/🟦️.ts`,
  `🧪️tests/🫂️presence-roster/🟦️.ts`
- `🧫️fixtures/🧾️learner-lifecycle/🔣️.json`, `🧫️fixtures/🏆️leaderboard/🔣️.json`, `🧫️fixtures/👥️shared-presence/🔣️.json`
- `🔮️oracles/🔣️.json`, `README.md`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (`🪪️identity-shapes` in the fixtures and tests lists)

**Updated — proctor**

- `🔨️modules/🎭️actors/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`, `🔨️modules/🔭️projections/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`,
  `🔨️modules/❓️queries/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`, `🔨️modules/⌨️cli/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`,
  `🔨️modules/🗄️storage/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`
- Edge's files, anchored edits only: `🔨️modules/🧩️instance/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`,
  `🔨️modules/🎚️config/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`
- `🧪️tests/🌐️end-to-end/🦀️.rs` (helper `envelope`/`target`, the lifecycle and operator tests, a new erase test; edge's
  region untouched except two `format!` literals clippy refused)
- `🏗️bootstrap/🟦️.ts`, `📦️packages/🦀️rust/📜️script.ts`, `📦️packages/🦀️rust/📋️project.json`, `README.md`

**Updated — elsewhere**

- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc` (five rows, two inputs)
- Ticket: `generate_quiz_vectors.py`

**Removed**

- No file. Removed from the code: the roster decider and its state in both cores, the `learner-recalled` event,
  `instance::admissible`/`ID_INVALID`/`KEY_BYTES` and their test, the verb `ready`, the projection key
  `LEADERBOARD_KEY`. `🗑️generated/domain/` is deleted.
