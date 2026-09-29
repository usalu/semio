# 🛂️ Proctor Report — `🎓️teaching/🛂️proctor`

The proctor is crate `teaching-proctor`, lib `proctor`, binary `proctor`, nx project `@teaching/proctor`. It is a
`ServerInstance` of the framework server product that runs the quiz lifecycle over one SQLite file and serves the built
site.

It is built on `semio-framework-quiz` (`quiz::decide_roster`, `evolve_roster`, `decide_learner`, `evolve_learner`,
`catalog_view`, `learner_view`, `run_view`, `leaderboard`, `catalog_issues`, `quiz_issues`) without duplicating any of
its rules.

**Status:** all green. 52 tests pass. Clippy (all targets) and rustdoc with `-D warnings` are clean. The dev proctor
was run manually against the real architecture catalog.

## 1. Architecture (decisions to know)

| Topic | Decision |
|---|---|
| Actors | `quiz-roster/roster` wraps `decide_roster`/`evolve_roster`. Each `quiz-learner/<learner>` wraps `decide_learner`/`evolve_learner`. State bytes are the serde JSON of `RosterState`/`LearnerState`. |
| Enrollment | A learner actor knows a learner only once `learner-registered` is in its own stream. The `EnrollmentSaga` is the instance's `Saga`. It relays every committed roster `learner-registered`/`learner-recalled` to `quiz-learner/<learner>` as the internal command `quiz.enroll-learner`. That command runs as service account `proctor`, and its idempotency key is `enroll:<tenant>:roster:<seq>`. The learner decider appends the fact, except a second registration: `evolve_learner` overwrites identity, so re-registering is refused to prevent takeover. This answers the core author's integration note. |
| Projections | A checkpointed `Projector` walks the database-wide commit order (`proctor_event.position`) after checkpoint `quiz`, in batches of 256. It writes every touched view plus the advanced checkpoint in **one transaction**. Per learner, the folded `LearnerState` is kept with its last folded stream `seq`, so a replayed batch is harmless. An in-memory mirror of the learner states makes the leaderboard recomputation O(learners) without decoding. The projector is deliberately **not** a `Saga`: `Saga::on_event` cannot report a failed write, and `drain_outbox` acknowledges the row regardless. |
| Read-your-writes | An outer middleware settles after every `POST /commands`: it drains the sagas until the outbox is quiet (reconciling the roster if an enrollment was refused), then catches the projections up. It catches up before every `POST /queries`. A supervisor repeats this every 500 ms, and once more on shutdown. So the identify → start-run → query sequence works immediately. |
| Catalog change | The catalog fingerprint is SHA-256 over the catalog bytes, each entry and each revision. If it differs from `quiz.meta/catalog`, boot resets and refolds the read models. This was observed live when another agent edited a quiz. |
| Identity and policy | No principal resolver exists, so every caller is `anonymous`. Template `quiz-learner` (auto-applied, and assigned to `anonymous`) admits `quiz.identify-learner` on `quiz-roster/roster`, the three learner commands on `quiz-learner/*`, `QueryAccess` on the four query kinds, and `EventDelivery`/`Subscription` on `stream:<catalog>/quiz-learner/*` only. The roster stream would leak handle → id and is therefore not readable. Template `quiz-proctor` (assigned to `service:proctor`) admits `quiz.enroll-learner`. |
| Gate | The outermost layer. Under `PROCTOR_TRUSTED_FORWARDING=proxy`, a first `X-Forwarded-Proto` other than `https` gets `403` with `x-semio-refusal: insecure-transport`. CORS is granted only to admitted origins: loopback in development, the allowlist in production. `OPTIONS` is answered by the gate. |
| Storage | One `rusqlite` 0.38 (bundled) connection behind `Arc<Mutex<Connection>>`, in WAL mode, `synchronous=FULL`, 5 s busy timeout, with a format row (`semio.teaching.proctor.sqlite` v1) that refuses foreign files. Events, receipts, outbox rows and deliveries (`proctor_outbox_delivery`) are insert-only. Snapshots and leases are replaceable, projections are derived, and sessions are deletable as the contract demands. |
| Framework fix | **`🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️.rs`**: `CommandBus::submit` never rehydrated an activation. After any restart it decided against empty state and appended `seq 1` onto an existing stream, which gave a `SequenceGap` and turned every command on an existing actor into `actorUnavailable`. The Place step now rehydrates a fresh activation from the snapshot plus `events_since` through the decider's `evolve`. A unit test covers it: `a_fresh_activation_rehydrates_from_the_durable_stream_before_deciding`. The fix also applies to hub's `CommandBus`. |

## 2. Files

**Created** under `🎓️teaching/🛂️proctor/`:

| Path | Content |
|---|---|
| `README.md` | Operator guide: production gating, environment, run, wire, backup/restore/rebuild, layout |
| `🔨️modules/🗄️storage/🦀️.rs` | `Database` (`open`, `open_read_only`, `memory`); `SqliteAuthorityStore` (plus `log_head` and `log_after`); `SqliteProjectionStore` (plus the atomic `commit`); `SqliteBlobStore`; `SqliteSessionStore` |
| `🔨️modules/📚️catalog/🦀️.rs` | `load_catalog`: SHA-256 revisions via `semio_framework_hash`, catalog plus nested quiz issues, fingerprint, `CatalogView` |
| `🔨️modules/🎚️config/🦀️.rs` | `ProctorConfig::from_environment`, production gating, origins, forwarding |
| `🔨️modules/🎭️actors/🦀️.rs` | `RosterDecider`, `LearnerDecider`, `ProctorDeciders` (`dyn_enum_close!`), `EnrollmentSaga`, the §9a envelope checks |
| `🔨️modules/🔭️projections/🦀️.rs` | `Projector`: `prepare`, `reset` and `catch_up` (progress and cancellation) |
| `🔨️modules/❓️queries/🦀️.rs` | `QuizQuery` for `quiz.catalog`, `quiz.learner`, `quiz.run` and `quiz.leaderboard` |
| `🔨️modules/🌐️site/🦀️.rs` | `SiteHost`: traversal guard, SPA fallback, content types, cache headers |
| `🔨️modules/🧩️instance/🦀️.rs` | `ProctorInstance`, `ProctorModule`, `ProctorResolvers`, manifest and templates; `Proctor` (`assemble`, `prepare`, `reconcile`, `settle`, `reset_projections`, `bind`); `BoundProctor::serve`; the gate and consistency middleware |
| `🔨️modules/⌨️cli/🦀️.rs` | `serve`, `check <catalog>` and `rebuild`. Stops on Ctrl+C, `SIGTERM`, Ctrl+Break, console close and shutdown. |
| `🔨️modules/*/🧪️tests/🔬️unit/🦀️.rs` | Unit tests, 9 files |
| `🏗️bootstrap/🦀️.rs` | Process entry (tokio multi-thread runtime) |
| `📦️packages/🦀️rust/{Cargo.toml, 🦀️.rs, 📋️project.json, 📜️script.ts}` | Glue with `#[path]` modules |
| `🧫️fixtures/{📚️catalog, ⚡️power, 🏠️homes}/🔣️.json` | Two-quiz fixture catalog with 3 badges: sorting + matching, and classification with spider profiles |
| `🧪️tests/🔬️conformance/🦀️.rs` | Framework storage laws, durable and in-memory |
| `🧪️tests/🌐️end-to-end/🦀️.rs` | HTTP end-to-end test driven by `ureq`, including a restart |

**Edited** with small anchored edits:

- Root `Cargo.toml`: the member line after `🌎️hub/📦️packages/🦀️rust`.
- `.gitignore`: `.🧬semio/🎓️teaching/`, after `.🧬semio/🌐hub/`, mirroring hub's dev data directory.
- `🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️.rs` and its unit test file: the rehydration fix above.
- `Cargo.lock`: updated by cargo.

**Ticket input kept:** `proctor_manual_drive.py`, the HTTP driver of the manual run.

## 3. Environment

| Variable | Meaning | Default |
|---|---|---|
| `PROCTOR_PORT` | TCP port | `8791` |
| `PROCTOR_BIND` | IP to bind (`0.0.0.0` in a container) | `127.0.0.1` |
| `PROCTOR_DATA` | Directory of `proctor.sqlite` | required |
| `PROCTOR_CATALOG` | Catalog `🔣️.json`. Quizzes are relative to it. | required |
| `PROCTOR_SITE` | Built site directory. It must contain `index.html`. | none |
| `PROCTOR_MODE` | `development` or `production` | loopback bind → development, else production |
| `PROCTOR_ALLOWED_ORIGINS` | Comma-separated `scheme://host[:port]` | loopback bind → loopback origins, else none |
| `PROCTOR_TRUSTED_FORWARDING` | `none` or `proxy` | `none` |

A network bind requires `PROCTOR_MODE=production`, `PROCTOR_ALLOWED_ORIGINS` and `PROCTOR_TRUSTED_FORWARDING=proxy`
together. Otherwise the boot is refused and the message names the missing variable. Development mode binds loopback
only.

`bun ./📜️script.ts dev` defaults `PROCTOR_DATA` to `<repo>/.🧬semio/🎓️teaching/proctor-dev` and `PROCTOR_CATALOG` to
`🎓️teaching/🏛️architecture/❓️quiz/🔣️.json`. Launcher values win.

## 4. Wire (actual JSON from the manual run against the dev proctor)

**Command.** Identify an anonymous learner with `POST /commands`. The `payload` is the UTF-8 bytes of
`{"type":"identify-learner","id":"0…03","learner":"0…a11","identity":{"kind":"anonymous"}}`:

```json
{"commandId":"00000000000000000000000000000003","kind":"quiz.identify-learner","version":1,"target":{"tenant":"architecture","kind":"quiz-roster","id":"roster"},"scope":"architecture","principal":{"kind":"anonymous"},"session":null,"device":null,"payload":[123,34,116,121,112,101,34,58,34,105,100,101,110,116,105,102,121,45,108,101,97,114,110,101,114,34,44,34,105,100,34,58,34,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,51,34,44,34,108,101,97,114,110,101,114,34,58,34,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,97,49,49,34,44,34,105,100,101,110,116,105,116,121,34,58,123,34,107,105,110,100,34,58,34,97,110,111,110,121,109,111,117,115,34,125,125],"causalFrontier":null,"clientHlc":{"millis":1727500000000,"counter":0},"expectedRevision":null,"idempotencyKey":"00000000000000000000000000000003","capabilityProof":null,"trace":{"trace_id":"manual","span_id":"manual"}}
```

The response (HTTP 200). The event payload decodes to
`{"type":"learner-registered","learner":"0…a11","identity":{"kind":"anonymous"},"at":1790609866012}`:

```json
{"status":"accepted","receipt":{"commandId":"00000000000000000000000000000003","actor":{"tenant":"architecture","kind":"quiz-roster","id":"roster"},"revision":3,"acceptedAt":{"millis":1790609866012,"counter":0}},"events":[{"stream":{"tenant":"architecture","kind":"quiz-roster","id":"roster"},"seq":3,"hlc":{"millis":1790609866012,"counter":0},"kind":"quiz.learner-registered","payload":[123,34,116,121,112,101,34,58,34,108,101,97,114,110,101,114,45,114,101,103,105,115,116,101,114,101,100,34,44,34,108,101,97,114,110,101,114,34,58,34,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,97,49,49,34,44,34,105,100,101,110,116,105,116,121,34,58,123,34,107,105,110,100,34,58,34,97,110,111,110,121,109,111,117,115,34,125,44,34,97,116,34,58,49,55,57,48,54,48,57,56,54,54,48,49,50,125]}],"frontier":null}
```

Other learner commands use the same envelope with these differences:

- `target` is `{"tenant":"architecture","kind":"quiz-learner","id":"<learner>"}`.
- The client may send `"principal":{"kind":"user","id":"<learner>"}`. The gateway overwrites it with the resolved
  principal, which is always anonymous.

Observed decoded outcomes:

| Command | Events |
|---|---|
| Recall (`identify-learner`, name `"ada lovelace"`, another learner id) | `[{"type":"learner-recalled","learner":"0…ada","at":…}]` |
| `start-run` | `[{"type":"run-started","learner":"0…ada","run":"0…5eed","quiz":"physics","revision":"bda31905…9397","seed":805373758,"at":…}]` |
| `record-answer`, first submission | `answer-recorded` |
| `record-answer`, same command id again | `{"status":"accepted",…,"events":[],"frontier":null}` |

A rejection (`submit-run` of an incomplete run):

```json
{"status":"rejected","receipt":{"commandId":"00000000000000000000000000000006","actor":{"tenant":"architecture","kind":"quiz-learner","id":"00000000000000000000000000000ada"},"revision":4,"acceptedAt":{"millis":1790609866043,"counter":0}},"reason":{"kind":"invalid","detail":"run-incomplete"},"notices":[]}
```

A malformed envelope is refused with a detail that starts `envelope-mismatch:` or `command-malformed:`.

**Query.** `POST /queries` for the leaderboard. Its `arguments` are the bytes of `{"type":"leaderboard"}`:

```json
{"queryId":"00000000000000000000000000002332","kind":"quiz.leaderboard","version":1,"scope":"architecture","principal":{"kind":"anonymous"},"arguments":[123,34,116,121,112,101,34,58,34,108,101,97,100,101,114,98,111,97,114,100,34,125],"consistency":{"kind":"authority"},"cursor":null}
```

It answers `{"kind":"snapshot","value":[…bytes of {"rows":[]}…],"frontier":null}`. Decoded `quiz.learner` and
`quiz.run` views from the same run:

```json
{"learner":"00000000000000000000000000000ada","identity":{"kind":"pseudonym","handle":"Ada Lovelace"},"runs":[{"run":"00000000000000000000000000005eed","quiz":"physics","status":"open","startedAt":1790609866020}],"badges":[],"best":{},"total":0.0}
{"run":"00000000000000000000000000005eed","learner":"00000000000000000000000000000ada","quiz":"physics","status":"open","sheet":{"quiz":"physics","seed":805373758,"title":{…},"description":{…},"tasks":[/* powers:sorting, energies:sorting, power-or-energy:classification */]},"answers":{},"startedAt":1790609866020}
```

An unknown learner or run answers `404 {"kind":"notFound","message":"not found: unknown-learner <id>"}`. A wrong scope
answers `404`. A kind that does not match its arguments answers `400`.

**Static site responses** (checked with `curl`):

| Request | Response |
|---|---|
| `GET /quiz/deep/link` | `200 text/html; charset=utf-8`, `cache-control: no-cache` |
| `HEAD /assets/app-B3xK9aQz.js` | `200 text/javascript; charset=utf-8`, `cache-control: public, max-age=31536000, immutable` |
| `GET /%2e%2e/%2e%2e/Cargo.toml` | `400` |
| `OPTIONS /commands` with `Origin: http://localhost:6061` | `204` with `access-control-allow-origin: http://localhost:6061` and `access-control-allow-credentials: true` |

## 5. Commands run and results

| Command | Result |
|---|---|
| `RUSTC_WRAPPER="" cargo test -p semio-framework-server` (after the authority fix) | lib 89 passed, including the new rehydration test; closed_ports 5; wire 3 |
| `RUSTC_WRAPPER="" cargo test -p teaching-proctor` | **unit 36 passed, conformance 14 passed, end_to_end 2 passed** |
| `RUSTC_WRAPPER="" cargo clippy -p teaching-proctor --all-targets --no-deps` | clean |
| `RUSTC_WRAPPER="" RUSTDOCFLAGS="-D warnings" cargo doc -p teaching-proctor --no-deps` | clean |
| `bun ./📜️script.ts test quick` (in `📦️packages/🦀️rust`) | 36 + 14 + 2 passed |
| `bun nx run @teaching/proctor:test --skip-nx-cache` | Successfully ran. The first attempt, on a cold build, hit the 15 s fundamental budget; see §7. |
| `bun ./📜️script.ts check` (architecture catalog) | `catalog architecture: 4 quizzes, 7 badges`, revisions printed, exit 0 |
| `bun ./📜️script.ts check <broken catalog>` | `[ERROR] /badges/0/rule/quiz quiz-unknown`, `/id slug-invalid`, `/introduction/paragraphs items-too-few`, `/title/en length-invalid`; exit 1 |
| `bun ./📜️script.ts rebuild` | `[INFO] rebuild: event 34 of 34 (100%)` then `[INFO] rebuild: projections current at event 34 (34 folded)` |
| `bun ./📜️script.ts verify taxonomy report --scope 🎓️teaching/🛂️proctor` (repo root) | 1 error: `🧪️tests/🌐️end-to-end` has no registered kind. The `📜️script.ts` router finding was fixed; the script now validates as `tool-metadata`. |

The conformance laws, each run against a durable file and against `:memory:`:

- **Authority:** receipts, sequence gaps, `events_since`, snapshots, outbox, leases.
- **Projections:** list and clear.
- **Blobs and sessions:** blob, session revocation.
- **Failing sink:** projection and session writes on a read-only reopen.
- **Restart:** the saga exactly-once law across a real reopen.

The end-to-end test covers:

- **Identify:** anonymous, a new pseudonym (normalised to `"Ada Lovelace"`), recall by name (`learner-recalled` for
  Ada), and `handle-invalid`.
- **Runs:** start-run, the sheet without solutions, perfect answers, and an idempotent retry (one `answer-recorded`
  in `/actors/…/events`).
- **Refusals:** the roster stream answers `403`. Commands are refused with `run-incomplete`, `answer-invalid`,
  `run-closed`, `unknown-quiz` and `unknown-learner`.
- **Results:** submission gives score 1 and the `perfect-power` and `sorter` badges; the run view shows `submitted`;
  the learner view shows best, total 100 and the badges; the leaderboard shows rank 1.
- **Restart:** stop, reopen the same file, get identical views, then play the second quiz. The stream sequence stays
  contiguous, the `complete` badge is awarded, and the total is 200.
- **Site test:** the gate refuses cleartext behind the proxy. It also covers the SPA fallback, cache headers, a 404
  for a missing asset, three raw traversal requests answering `400`, and the CORS preflight granting a loopback origin
  and ignoring a foreign one.

## 6. Manual run (real server logs)

`PROCTOR_SITE=<ticket>/🗑️generated/proctor/site bun ./📜️script.ts dev`, then `proctor_manual_drive.py`:

```
[INFO] proctor 0.1.0 mode development, data C:\git\semio\.🧬semio\🎓️teaching\proctor-dev, catalog C:\git\semio\🎓️teaching\🏛️architecture\❓️quiz\🔣️.json, site …/🗑️generated/proctor/site, cross-origin loopback-development, trusted forwarding none
[INFO] catalog architecture: 4 quizzes, 7 badges
[INFO] projections were built against another catalog; rebuilding them
[INFO] catch-up: projections current at event 0 (0 folded)
[INFO] proctor listening on http://127.0.0.1:8791
[DEBUG] instance: 200 teaching-proctor
[DEBUG] identify-pseudonym: 200 accepted ['learner-registered']
[DEBUG] identify-recall: 200 accepted ['learner-recalled']
[DEBUG] identify-anonymous: 200 accepted ['learner-registered']
[DEBUG] start-run: 200 accepted ['run-started']
[DEBUG] record-answer: 200 accepted ['answer-recorded']
[DEBUG] record-answer-retry: 200 accepted []
[DEBUG] submit-incomplete: 200 rejected [] run-incomplete
```

The data directory held `proctor.sqlite`, `-wal` and `-shm`, which confirms WAL mode. I then killed the process with
`taskkill /F` as a simulated crash and restarted it:

```
[INFO] projections were built against another catalog; rebuilding them      ← a content agent had just edited heating/🔣️.json
[INFO] catch-up: event 8 of 8 (100%)
[INFO] catch-up: projections current at event 8 (8 folded)
[INFO] proctor listening on http://127.0.0.1:8791
[DEBUG] after crash+restart learner: {'kind': 'pseudonym', 'handle': 'Ada Lovelace'} [('physics', 'open')]
[DEBUG] after crash+restart run: open ['powers']
[INFO] break received; stopping                                              ← Ctrl+Break sent to the process group
[INFO] proctor stopped
```

After the clean stop only `proctor.sqlite` remained; the WAL was checkpointed.

Afterwards, the shared dev database held 36 events from **another agent's browser session**. That session used the
React client against its own dev proctor on the same directory. It went through identify, start-run, 20
`record-answer` events (outbox-coalesced), submit, two `badge-awarded` and a recall, which shows the §9a mapping
working with the real client.

`[DEBUG]` lines come only from the ticket driver script. The Rust code has no `[DEBUG]` output.

## 7. Open issues and hand-offs

1. **Taxonomy (site-infra, §12).** Register a kind for `🎓️teaching/🛂️proctor/🧪️tests/🌐️end-to-end`. It is the only
   finding of `verify taxonomy report --scope 🎓️teaching/🛂️proctor`.
2. **Launch rows and root scripts (site-infra).** These are not added by me. Suggested rows:
   - `🛠️dev🎓️teaching🛂️proctor` → `bun nx run @teaching/proctor:dev`, env `PROCTOR_PORT=8791`, with a
     `serverReadyAction` on `http://127.0.0.1:8791`.
   - `🧪️test🎓️teaching🛂️proctor` → `:test`.
   - `📦️build🎓️teaching🛂️proctor` → `:build`.
   - A `check` row and a `rebuild` row.
3. **Deployment (site-infra) — already consistent.** I checked
   `🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy/{Dockerfile,compose.yaml}`, and it matches this binary:
   - `cargo build --release --package teaching-proctor --bin proctor`.
   - `ENTRYPOINT tini -- proctor` with `CMD ["serve"]`. The proctor handles `SIGTERM`.
   - The `PROCTOR_*` names and production triple.
   - `HEALTHCHECK` on `/instance` with `X-Forwarded-Proto: https`.
   - One data volume.
4. **Hub not re-tested.** The rehydration fix changes `CommandBus` behavior for hub too; it now rehydrates after a
   restart where it previously hit `SequenceGap`. `cargo test -p semio-hub` could not run: the machine hit
   `STATUS_NO_MEMORY` compiling hub dependencies while other agents built. The server crate's own suites are green.
5. **Budget flake.** `runCargoTestBudgeted`'s no-nextest fallback warm-builds with `cargo build --tests`, but
   `cargo test` can still compile inside the 15 s assertion budget on a cold cache. The first nx run failed this way;
   the rerun passed in about 1 s of tests. This is library-wide, not proctor-specific. Installing `cargo-nextest`
   avoids it.
6. **One serving process per `PROCTOR_DATA`.** Documented in the README. I first tried SQLite
   `locking_mode=EXCLUSIVE` to enforce it, but the framework's saga-restart law requires a second open while the first
   handle is alive, so enforcement was dropped. `proctor rebuild` against a directory a live proctor serves is safe
   (the fold is idempotent and the serving projector's checkpoint is advanced by the rebuild) but not recommended.
   During this session another agent's dev proctor was using `.🧬semio/🎓️teaching/proctor-dev/`; my manual rebuild
   ran against it harmlessly.
7. **No Protocol v2 case.** No `🥒️.feature` case exists for the proctor wire. The language-agnostic evidence is the
   HTTP end-to-end test with a third-party client (`ureq`) plus the framework conformance laws. `sha2` is the
   third-party oracle for revisions. A Protocol v2 case would belong to the conformance work package (oracle
   registry).
8. **Run views of a revised quiz.** `quiz::run_view` recomputes the sheet from the *current* quiz. A run started
   against an older revision therefore shows the new quiz's sheet until it is voided on the next start or submit.
   This is core behavior (see the core report).

## 8. Follow-up fixes (coordinator round 2)

All three requested fixes are done, verified on a running proctor, and covered by tests. The proctor owner has 0
Protocol v2 layout breaches.

| # | Fix | Where |
|---|---|---|
| 1 | **Cache headers follow the build layout, not the hash alphabet.** `is_hashed` is replaced by `cache_control(relative path)`: any `*.html` is `no-cache`; any file inside `assets/` (Vite's `build.assetsDir`) is `public, max-age=31536000, immutable`; anything else is `public, max-age=3600`. This covers dash hashes (`🌐️-Djvsi-pa.js`), `pdf.worker.min-iDqQPrd3.mjs` and the css. Copied public files such as `🖼️assets/🔤️fonts/…woff2` get the short cache. `404.html` and `🌐️.html` are now revalidated too, since stale HTML pointing at superseded hashes is the failure mode to avoid. | `🔨️modules/🌐️site/🦀️.rs` |
| 2 | **No exe lock on Windows.** `dev`, `check` and `rebuild` now go through `🏗️bootstrap/🟦️.ts::runProctor`. It builds with `cargo build --message-format=json-render-diagnostics`, takes the executable path from Cargo's `compiler-artifact` message, and **copies** it (not hard-links) to `.🧬semio/🎓️teaching/proctor-bin/proctor-<pid>-<ms>[.exe]` (git-ignored). It runs that copy with stdio inherited and deletes it on exit. The launcher outlives `SIGINT`, `SIGTERM`, `SIGHUP` and `SIGBREAK` (filtered to the signals the OS knows) and forwards them on POSIX. On Windows the child receives the console event itself. A copy left by a killed launcher is removed at the next start (a copy still running refuses removal and is kept). This is one code path on Windows, macOS and Linux. `📜️script.ts` stays a thin router and validates as `tool-metadata`. | `🏗️bootstrap/🟦️.ts` (new), `📦️packages/🦀️rust/📜️script.ts` |
| 3 | **405 for other methods.** Any non-`GET`/`HEAD` request to the site answers `405 Method Not Allowed` with `Allow: GET, HEAD` and `{"kind":"methodNotAllowed","message":…}`. This holds with or without `PROCTOR_SITE`. Gateway routes keep axum's own `405` with their `Allow` (for example `GET /commands` gets `Allow: POST`). | `🔨️modules/🌐️site/🦀️.rs` (`method_not_allowed`), `🔨️modules/🧩️instance/🦀️.rs` (fallback) |

Other changes in this round:

- **Tag field:** `LeaderboardRow.learner` became `tag`. The core agent had already adapted the projection and
  end-to-end assertions; both pass.
- **Line endings:** my earlier Python edits had written CRLF into nine proctor files. All proctor files are LF again.
- **README:** new "Static site" table and a paragraph on the private-copy launcher.

### Verification (run)

| Check | Result |
|---|---|
| `RUSTC_WRAPPER="" cargo test -p teaching-proctor` | unit **38** passed (new `cache_policy_follows_the_build_layout_not_the_hash_alphabet`, `other_methods_are_not_allowed`), conformance **14**, end_to_end **2** (now also asserting the dashed-hash asset is immutable, `🖼️assets/…woff2` gets the short cache, and `POST`/`PUT`/`DELETE` get `405` with `allow: get, head`) |
| `bun ./📜️script.ts test` | same: 38 + 14 + 2 passed |
| `cargo clippy -p teaching-proctor --all-targets --no-deps` | clean |
| `RUSTDOCFLAGS="-D warnings" cargo doc -p teaching-proctor --no-deps` | clean |
| `tsc` over `🏗️bootstrap/🟦️.ts` and `📜️script.ts` (root tsconfig) | 0 errors in these files. The 47 reported errors are all in pre-existing framework files reached through the library barrel. |
| `verify taxonomy report --scope 🎓️teaching/🛂️proctor` | `clean=true errors=0` |
| `🧪️test` → `contract --owner 🎓️teaching/🛂️proctor` | 0 breaches in `🎓️teaching`. The report is repo-wide: 5,046 breaches elsewhere, including the quiz crate's inline tests. I proved the scan covers the proctor tree with a temporary probe file containing an inline `#[test]`; it was flagged, then removed. |

The exe-lock fix was checked live:

1. `bun ./📜️script.ts dev` (port 18791, private data dir, the real Vite `dist`). The process list shows
   `…\.🧬semio\🎓️teaching\proctor-bin\proctor-46076-1790612814878.exe` serving.
2. While it served, I changed `🌐️site/🦀️.rs` and ran `bun ./📜️script.ts check`. Cargo printed
   `Compiling teaching-proctor … Finished`, check printed the catalog, and it exited 0. Before the fix this relink
   failed with `Access is denied`. Check's own copy was deleted; the dev copy was kept.
3. I sent Ctrl+Break to the dev copy. It logged `[INFO] break received; stopping` and `[INFO] proctor stopped`, and
   `proctor-bin/` was empty afterwards.
4. The first attempt, before `SIGBREAK` was in the launcher's signal list, showed that the launcher itself died on
   Ctrl+Break and left its copy behind. The next run removed that copy at start, and the fix above closes the gap.

Live headers against the real build, with an extra `assets/🌐️-Djvsi-pa.js`:

| Request | Answer |
|---|---|
| `/assets/%F0%9F%8C%90%EF%B8%8F-Djvsi-pa.js` | `200`, `immutable` |
| `/assets/🌐️-B0ABbSUq.css` | `200`, `immutable` |
| `/assets/pdf.worker.min-iDqQPrd3.mjs` | `200`, `immutable` |
| `/`, `/index.html`, `/404.html`, `/quiz/run/deep` | `200`, `no-cache` |
| `/favicon.svg` and `/🖼️assets/🔤️fonts/…/compressed.woff2` | `200`, `public, max-age=3600` |
| `POST /quiz` | `405`, `allow: GET, HEAD`, `{"kind":"methodNotAllowed","message":"method not allowed: POST /quiz (the site answers GET, HEAD)"}` |
| `DELETE /assets/x.js` | `405`, `allow: GET, HEAD` |
| `GET /commands` | `405`, `allow: POST` (the gateway's own) |

`bun ./📜️script.ts rebuild` through the launcher refolded the shared dev directory: `event 256 of 285 (89%)`, then
`event 285 of 285 (100%)`, exit 0.

### Still open

- **Launch rows (site-infra):** `📓️audit-agents-rules.md` §1 notes that `@teaching/proctor:check` and
  `@teaching/proctor:rebuild` have no `.vscode/launch.json` row and no root `package.json` script. Launch rows are
  site-infra's registration, so I left them unchanged.
