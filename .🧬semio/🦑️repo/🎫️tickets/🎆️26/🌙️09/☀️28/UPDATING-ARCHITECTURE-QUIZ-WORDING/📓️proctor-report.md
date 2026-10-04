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

## 9. 2026-09-29 — API-only proctor for the CDN site

This implements design §14 and `📓️explore-cdn-docker-deploy.md` §3 and §5.2. The site is now served from a CDN at
`https://quizzes.architektur-und-technologie.de`. The proctor is API only and runs at
`https://proctor.quizzes.architektur-und-technologie.de`.

| # | Change | Where |
|---|---|---|
| 1 | **Static hosting removed completely.** The `🔨️modules/🌐️site` module and its tests are deleted, along with `PROCTOR_SITE` (the variable, the config field and the startup log field), the `site` argument of `Proctor::assemble`, the SPA and cache-header logic, the site `405` rule, and the README "Static site" section. Every path the gateway does not route answers its JSON `404` `{"kind":"notFound","message":"not found: GET /"}`, whatever the method. A known route with another method keeps axum's `405` with its `Allow`. `assemble` is now `(profile, catalog, gate)`. | `📦️packages/🦀️rust/🦀️.rs`, `🎚️config`, `🧩️instance`, `⌨️cli`, `Cargo.toml` description, README |
| 2 | **Preflight max-age.** The gate answers every `OPTIONS` itself via `preflight()`. For an admitted origin it sends `204`, the echoed origin, credentials, methods, `content-type` and **`Access-Control-Max-Age: 7200`** (`PREFLIGHT_MAX_AGE`, the Chromium ceiling), plus `Vary: Origin`. A foreign origin gets `204` with `Vary: Origin` and neither the origin grant nor the max-age. `grant()` now reports whether it granted. | `🔨️modules/🧩️instance/🦀️.rs` |
| 3 | **Tests.** The unit test `the_cross_origin_grant_follows_the_policy` is extended (methods, headers, no grant without an `Origin`). The new unit test `a_granted_preflight_is_cacheable_and_a_refused_one_is_not` covers max-age. The e2e site test is replaced by `the_api_serves_the_cdn_site_across_origins_behind_the_gate`, detailed below the table. | `🧩️instance/🧪️tests/🔬️unit/🦀️.rs`, `🧪️tests/🌐️end-to-end/🦀️.rs` |
| 4 | **README.** Documents API-only operation, the Cross-origin API (preflight table), the environment table with a production column (allowlist = site origin `https://quizzes.architektur-und-technologie.de`, host `proctor.quizzes…`), Caddy in front from `🚀️deploy/`, a Docker backup row (`docker compose stop proctor`, copy `proctor.sqlite` out of `/srv/quiz/data`, then `start`), and the layout without `🌐️site`. The startup line now lists the admitted origins: `cross-origin allowlist https://quizzes.architektur-und-technologie.de`. | `README.md`, `🎚️config` (`CrossOriginPolicy::describe`) |
| 5 | **Test origins renamed.** `quizze.example` is now `quizzes.example` everywhere in the proctor tests. No `quizze.` string remains in the proctor tree. | `🎚️config` and `🧩️instance` unit tests |
| 6 | **`Quiz.emoji`.** The Rust core agent updated the crate types, both fixture quizzes (`⚡`, `🏠`) and the catalog unit test's inline quiz. I did not touch those. The catalog view now carries the emojis: live, `quiz.catalog` answered `physics 🧲, heating 🔥, cooling ❄️, demand 📊`. | core agent |

The new end-to-end test `the_api_serves_the_cdn_site_across_origins_behind_the_gate` runs the proctor with an allowlist
of `https://quizzes.example` behind a trusted proxy and checks the following:

- A cleartext request to `/instance` gets `403` with `insecure-transport`.
- `/`, `/index.html`, a deep route and an asset path each get a JSON `404` with `notFound`.
- The preflights `OPTIONS /commands` and `OPTIONS /queries` from the site carry the echoed origin, the methods,
  `content-type`, `max-age: 7200` and `vary: origin`. From a foreign origin they carry neither the origin grant nor
  the max-age.
- Real cross-origin `POST /commands` (identify) and `POST /queries` (learner view) calls with `Origin: <site>` and no
  credentials are accepted and carry the echoed origin.
- A foreign-origin `POST /queries` gets no origin grant and `Vary: Origin`.

### Verification (run)

| Check | Result |
|---|---|
| `RUSTC_WRAPPER="" cargo test -p teaching-proctor` | unit **33** passed (38 − 6 site tests + 1 preflight test), conformance **14**, end_to_end **2** |
| `cargo clippy -p teaching-proctor --all-targets --no-deps` | clean |
| `RUSTDOCFLAGS="-D warnings" cargo doc -p teaching-proctor --no-deps` | clean |
| `verify taxonomy report --scope 🎓️teaching/🛂️proctor` | `clean=true errors=0` |
| `🧪️test` → `contract --owner 🎓️teaching/🛂️proctor` | 0 proctor breaches (5,075 elsewhere in the repo) |
| Live: `bun ./📜️script.ts dev` with `PROCTOR_ALLOWED_ORIGINS=https://quizzes.architektur-und-technologie.de` | The four live checks are listed below. |

Live results:

- **Startup line:** `[INFO] proctor 0.1.0 mode development, … cross-origin allowlist https://quizzes.architektur-und-technologie.de, trusted forwarding none`.
- **Site-origin preflight:** `OPTIONS /commands` from the site answered `204` with the echoed origin, credentials,
  methods, `content-type` and `access-control-max-age: 7200`. From `https://evil.example` it answered `204` without an
  origin grant or max-age.
- **Unknown path:** `GET /` answered `404 {"kind":"notFound","message":"not found: GET /"}`.
- **Cross-origin query:** `POST /queries` from the site answered `200` with an echoed origin grant.

Ctrl+Break ended the run with `[INFO] break received; stopping` and `[INFO] proctor stopped`, and the private binary
copy was removed.

A preflight to a POST-only route also carries axum's `allow: POST`. Axum's method router adds it when the gated
fallback path answers. Browsers ignore it on a preflight.

### Hand-offs (not mine)

- **Deploy files (site-infra).** `🚀️deploy/Dockerfile` still sets `PROCTOR_SITE=/srv/quiz/site` and copies the site
  into the image, and `Dockerfile`/`compose.yaml` still name `https://quizze.…` as the allowlist. The proctor now
  ignores `PROCTOR_SITE`; nothing reads it. But the variable, the bun stage and the old origin must go per §5.2, and
  the allowlist must become `https://quizzes.architektur-und-technologie.de`. I did not edit those files.
- **Docker backup commands.** The `docker compose stop/cp/start` backup row was not run: no Docker daemon answered on
  this host (`docker ps` hung).
- **Launch rows.** The rows for `@teaching/proctor:check`/`:rebuild` are still open for site-infra (see §8).

## 10. 2026-09-29 presence — shared presence and cursors (design §15)

Every learner now shares their presence and cursor with the other learners. The framework half is domain-neutral:
nothing in `🧰️framework/🛍️products/🖥️server` knows about quizzes. The proctor half decides which rooms exist and
which states are admitted, using the quiz core's rules.

### Framework (`semio-framework-server`, `@semio-tech/framework-server`)

- **Route.** `GET /scopes/{scope}/presence/ws?surface=` is a WebSocket with subprotocol `semio.presence.v1`. It is
  mounted in `base_router`, listed in the wire fixture's route table and in `SERVER_ROUTES`.
- **Contract (`🧬️contract`).** `PresenceEntry {session, colour, surface, state}` and
  `PresenceFrame = welcome | state | batch | refused` (tag `type`, camelCase). `OpaqueJson` is re-exported for the
  opaque state.
- **Rooms (`📡️gateway`, region `PresenceRoom`).**
  - `PresenceRooms` keeps the latest state per session.
  - One ticker per room drains `changed`/`left` into one `batch` on `presence_lane(scope)`. The ticker starts on
    the first join and ends atomically when the room is empty.
  - `PresenceSettings` has these defaults: tick 100 ms, idle 60 s, keepalive ping 20 s, state ≤ 2048 bytes, ≤ 30
    states/s (the excess is dropped silently), surface ≤ 64 characters.
  - Session ids are 32 random hex characters, generated by the server. Colours come from the existing `Presence`
    registry, which also handles join and leave.
- **Admission.**
  - The `Origin` must pass the instance's `OriginAdmission` hook (`ServerBuilder::origin_admission`) when both are
    present.
  - The surface length is checked (`400`).
  - `PolicyPoint::Subscription` `join` on resource `presence` is required (`403`). `publish` decides whether states
    are taken; without it they are answered `refused forbidden`.
  - Every module's new default method `ServerModule::presence_admission(scope, state) -> Result<(), String>` may
    refuse a state. The default admits it, so the hub is unchanged. No associated type was added.
- **Socket loop.**
  - The socket subscribes to the lane before joining, then sends `welcome`, which includes the session itself.
  - A text frame goes through `admit_state`: size, then rate, then shape. Then the modules' admission, then the room.
  - A binary frame is answered with `refused frame-invalid`.
  - A lagged lane resends `welcome` with the current roster.
  - A socket that sends nothing for 60 s (no frame, no pong) is closed. On close, the session leaves the room, the
    registry and the kick map.
- **TS twin (`🟦️.ts`).** `PRESENCE_PROTOCOL`, `PresenceEntry`, `PresenceFrame`, `decodePresenceFrame` (throws
  `WireError` with the field path), `encodePresenceFrame`, `presenceLane(scope)`,
  `presenceSocketUrl(baseUrl, scope, surface)` (the scope is percent-encoded, the surface is a query parameter), and
  `ServerClient.presenceSocketUrl`.
- **Wire fixture.** One route and four vectors (`presence-welcome`, `presence-state`, `presence-batch`,
  `presence-refused`). Both twins round-trip them byte for byte, and both assert that every frame type has a vector.
- **Clippy, rustdoc and a fix along the way.** The crate had pre-existing `clippy -D warnings` and `rustdoc -D warnings`
  failures, and I fixed them all:
  - `AuthorityDirectory::activate` now borrows its key, so a command no longer clones it.
  - `build` no longer holds the policy lock across `templates().await`.
  - Unneeded qualifications were removed, a counter loop and a slice clone were cleaned up in the test instance and
    the conformance laws, and five broken or redundant intra-doc links were repaired.

### Proctor

- **`🔨️modules/👥️presence` (new).** `Rooms::of(catalog)` builds the room scopes with the core's `roster_scope` and
  `room_scope`: the roster `<catalog>`, `<catalog>/{introduction,home,leaderboard}`, and `<catalog>/quiz/<quiz>` for
  every catalog quiz. `Rooms::admit(scope, state)` works like this:
  - The roster admits a `PresenceState` that passes `presence_problem`, whose place names a quiz of this catalog and
    a task of that quiz.
  - A place room admits a `CursorState` that passes `cursor_problem`.
  - Any other scope gets `scope-unknown`.
  - A state of the wrong type gets `state-invalid`. Because the schema's `deny_unknown_fields` applies, a cursor state
    can never smuggle an answer.
  - A core issue gets `<code> <pointer>`, for example `tag-invalid /tag`.
- **`🧩️instance`.**
  - `ProctorModule::new(catalog)` holds the rooms and implements `presence_admission`.
  - The new template `quiz-presence` grants Subscription `join` and `publish` on `presence`. It is assigned to
    `anonymous` with `assign_scoped` once per room scope. Every caller is `anonymous`, so every principal may join
    exactly the catalog's rooms, and every other scope is closed by policy (`403` before the upgrade).
  - `assemble(profile, catalog, gate, presence)` sets `.presence(settings)` and
    `.origin_admission(gate.origins.admits)`. In production only the site origin passes; in development any loopback
    origin (the Vite proxy has `ws: true`).
- **`🎚️config`.** `PROCTOR_PRESENCE_TICK_MS` takes 10 to 1000 ms (default 100). `ProctorConfig::presence()` gives the
  settings, and the startup line reports the tick.
- **Fix along the way (`🔭️projections`).** `prepare` now answers `true` only when read models of *another* catalog were
  dropped. A fresh data directory is stamped silently, so a first boot no longer logs "projections were built against
  another catalog".
- **README.** Adds a "Presence (design §15)" section (the room table, the protocol, admission and privacy), a new
  environment row, and layout rows.

### Tests (all run)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-server` | lib 95 passed (6 new presence tests: coalescing, size/rate/shape limits, session ids, admission by grant/surface/origin, two live sessions incl. leave, idle close), closed_ports 5, wire 4 (new `every_presence_frame_has_a_vector`) |
| `cargo clippy -p semio-framework-server --all-targets --features conformance --no-deps -- -D warnings` | clean |
| `RUSTDOCFLAGS=-D warnings cargo doc -p semio-framework-server --no-deps` | clean |
| `bun ./📜️script.ts test` (server TS package) | 15 passed, incl. new "carries a vector for every presence frame and refuses a malformed one", lane and URL assertions |
| `tsc --strict --noEmit` on the server `🟦️.ts` (bun + vitest types) | 0 errors |
| `cargo test -p teaching-proctor` and `bun ./📜️script.ts test` | lib 39 passed (4 presence, 1 config tick, 1 instance presence policy/origin/admission), conformance 14, end_to_end 4 |
| `cargo clippy -p teaching-proctor --all-targets --no-deps -- -D warnings`, `cargo doc -p teaching-proctor --no-deps` (`-D warnings`) | clean |
| `bun ./📜️script.ts contract --owner "🎓️teaching/🛂️proctor"` | 0 breaches under `🎓️teaching` or `🖥️server`; the full set `⚡️cache/breaches/testing.json` also has none |
| `verify taxonomy report --scope 🎓️teaching/🛂️proctor` | `clean=true errors=0` |

**End-to-end (`🧪️tests/🌐️end-to-end`).** Two new tests drive the real gateway with an independent WebSocket client,
`tungstenite` 0.26 (sync, dev-dependency; it was already in the lock through the hub).

`learners_share_presence_and_cursors_through_the_gateway` covers:
- The first `welcome` holds only the session itself, with a null state.
- A second learner's `welcome` roster carries the first learner's shared state, with a distinct colour.
- Each learner sees the other join and share.
- Refusals: `tag-invalid /tag`, `state-invalid` (missing fields), `quiz-unknown /place/quiz`, and `state-invalid` for a
  presence state in a quiz room.
- Ten cursor moves inside one tick reach the watcher in at most two batches, ending with the last position.
- `403` for another catalog's roster and for an unknown quiz's room, and `400` for a 65-character surface.
- A `left` batch after each close.

`a_production_proctor_opens_presence_to_the_site_origin_only` covers:
- Cleartext behind the proxy is refused with `403 insecure-transport`.
- A foreign origin, and a loopback origin, are refused with `403`.
- The site origin with `X-Forwarded-Proto: https` joins, shares and sees its own batch.

**Runtime across languages.** `proctor_presence_smoke.ts` (ticket folder) runs a private copy of the built Rust
`proctor serve` (fixture catalog, `PROCTOR_PRESENCE_TICK_MS=50`) and drives it with two Bun WebSockets through the
**TypeScript twin** (`presenceSocketUrl`, `decodePresenceFrame`, `encodePresenceFrame`). It logged, in order:
- The startup line with `presence tick 50 ms`.
- `welcome` frames with colours 0 and 1.
- A batch carrying Ada's `PresenceState`.
- `refused quiz-unknown /place/quiz`.
- A batch with Ada in `left`.
- `[DEBUG] presence smoke passed`.

The `[DEBUG]` lines come only from that script.

**Observed.** The server stores the opaque state as a `serde_json::Value`, so object keys come back sorted. Clients must
compare states structurally, not as strings.

### Files

- Framework:
  - `🧰️framework/🛍️products/🖥️server/🔨️modules/{🧬️contract,📡️gateway,🎭️authority,🛡️policy}/🦀️.rs`
  - `…/🔨️modules/{📡️gateway,🎭️authority}/🧪️tests/🔬️unit/🦀️.rs`
  - `…/🧪️tests/{🧩️instance,🔬️conformance,🔬️wire}/🦀️.rs`
  - `…/🧪️tests/🔬️wire/🟦️.ts`
  - `…/🧫️fixtures/🔌️wire/🔣️.json`
  - `…/🟦️.ts`
- Proctor:
  - `🎓️teaching/🛂️proctor/🔨️modules/👥️presence/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}` (new)
  - `…/🔨️modules/{🎚️config,🧩️instance,🔭️projections}/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`
  - `…/🔨️modules/⌨️cli/🦀️.rs`
  - `…/🧪️tests/🌐️end-to-end/🦀️.rs`
  - `…/📦️packages/🦀️rust/{🦀️.rs,Cargo.toml}`
  - `…/README.md`
- Also: `Cargo.lock` (the `tungstenite` dev-dependency of `teaching-proctor`) and the ticket script
  `proctor_presence_smoke.ts`.

### Hand-offs (not mine)

- **Client (react agent).**
  - Open the roster socket and the room socket with `presenceSocketUrl` and the core's `rosterScope`/`roomScope`.
  - Keep the subprotocol `semio.presence.v1`.
  - Treat `refused` reasons as `<code>[ <pointer>]`.
  - Compare states structurally.
  - Browsers answer the server's pings, so only dead sockets hit the 60 s idle close.
- **Hub.** It inherits the default admission and holds no presence grant, so its presence sockets are closed by
  policy and it sets no origin hook. `cargo check -p semio-hub` was not run, because it runs out of memory on this
  host. The hub uses none of the changed signatures (`activate`, `Subscription::next`, `ServerState` fields); I
  checked by grep.
- **Taxonomy of the server product.** `verify taxonomy report --scope 🧰️framework/🛍️products/🖥️server` reports 4
  `directory-kind-unresolved` errors for directories that existed before this change: `🧪️tests/🔒️closed-ports`,
  `🧪️tests/🔬️wire`, `🧪️tests/🧩️instance` and `🧫️fixtures/🔌️wire`. They need kinds in `🔣️taxonomy.json`.
- **Deploy (site-infra).** Caddy's `reverse_proxy` already forwards the upgrade. `PROCTOR_PRESENCE_TICK_MS` is
  optional (default 100).

## 11. 2026-09-29 night — watching, thinking rooms and the crowd (design §16, §17)

### Framework: watching (domain-neutral)

- **Contract.** `PresenceFrame` gains two frames:
  - `Watch {scopes, intervalMs}` travels client → server.
  - `Watched {scope, entries, left, snapshot?}` travels server → client. `snapshot` is sent only when true.
- **Admission.**
  - `admit_frame` replaces `admit_state`. It admits `state` and `watch`; both count against the 2 KiB size limit and
    the 30/s rate.
  - `admit_watch(state, principal, scopes)` allows at most `max_watch_scopes` scopes (16), otherwise
    `watch-too-many`. Each scope needs `PolicyPoint::Subscription` `watch` on `presence`, checked for the principal
    resolved at the handshake. This is the same scoped policy that admits joins.
  - A refusal answers `forbidden <scope>` and leaves the previous watch set unchanged.
  - `admit_presence` now returns `PresenceGrant {principal, may_publish}`.
- **Coalescing.**
  - `Watching` keeps, per watched scope, the latest entry per session and the sessions that left. `flush()` sends one
    `watched` frame per changed scope.
  - `snapshot()` sends the whole roster and discards what was pending for that scope. `watch()` replaces the set and
    returns the newly added scopes.
  - `watch_interval` clamps the requested interval to between the tick and `max_watch_interval` (60 s).
- **Socket.**
  - Each watched room gets one forwarder task. It subscribes before the snapshot is taken, parses batches off the
    socket's task and relays them over a bounded channel (64).
  - When a forwarder falls behind, the scope gets a fresh snapshot.
  - The interval restarts only when it changes. An empty watch stops the timer.
  - Forwarders are aborted when a scope is unwatched and when the socket closes. The unit test shows every lane is
    released after close.
- **Wire and TypeScript twin.**
  - New fixture vectors: `presence-watch`, `presence-watched-snapshot`, `presence-watched`.
  - Both twins assert every frame type has a vector, and that `snapshot` is omitted when unset.
  - The TS `PresenceFrame` union, `decodePresenceFrame` and `encodePresenceFrame` cover `watch` and `watched`.
    `intervalMs` must be a safe integer; `snapshot` is optional and must be a boolean when present.
- **Unit tests (new).**
  - Coalescing per interval, including a later entry superseding a departure, a snapshot absorbing what was pending,
    unwatched scopes being ignored, and kept scopes keeping their pending changes.
  - Interval clamping, and `watched` refused as a client frame.
  - Per-scope admission: closed by default, a `join` grant is not a `watch` grant, more than 16 scopes refused.
  - A live test with three sessions:
    - one snapshot per watched scope, in scope order;
    - 8 states over about 80 ms reach an 80 ms watcher at most once per interval;
    - watching never joins the watched room;
    - a refused watch keeps the previous set, which the following `left` frame proves;
    - an empty watch goes silent;
    - every lane is released after close.
  - Ran 6 times in a row: 6 of 6 passed.

### Proctor

- **Rooms (`👥️presence`, rewritten).** Following §16 and §17, the rooms are:
  - the roster;
  - four pages: introduction, home, leaderboard, badges;
  - one quiz room `<catalog>/quiz/<quiz>` per quiz;
  - one thinking room `<catalog>/quiz/<quiz>/thinking` per quiz.

  Admission first applies the core's rule for the room's state type (`presence_problem`, `cursor_problem`,
  `thinking_problem`). It then checks that the state names only what the catalog renders:
  - A place must name a quiz of the catalog and a task of that quiz.
  - Pages allow no drag. In a quiz room, `drag.item` must be an item of that quiz.
  - Drafts must name tasks of the quiz, with an answer of the task's kind (`kind-mismatch`), and only its items,
    categories and dimensions (`id-unknown`).
  - Matching drafts may assign only values a card of that dimension shows (`value-unknown`). The core defines these
    drafts as `ThinkingMatchingAnswer.values`, so card indices never reach peers.
- **Policy.** `quiz-presence` now grants `join`, `publish` and `watch`. It is still assigned per room scope, so the
  thinking rooms are included and every other scope stays closed.
- **Crowd.**
  - **Tally (`👪️crowd`, new).** `CrowdTally` folds each `RunResult` in O(its size). It keeps classification counts
    per task, item and category; sorting counts and running sums per item, the sum stored as exact `f64` bits because
    serde_json's default float parsing is not guaranteed to round-trip; and matching counts per dimension, item and
    value. Values are keyed by the core's `json_number_text`.
  - **Same rules as the core.** Like `crowd_view`, the tally counts only the first task of each id and kind, and the
    first item and dimension of each id. `view(quiz)` materializes against the current definition.
  - **Projection.**
    - The projector folds `RunSubmitted` results into per-quiz tallies inside the same checkpointed batch
      transaction as every other view, deduplicated by learner sequence.
    - It writes `quiz.crowd-tally/<quiz>` and `quiz.crowd/<quiz>`.
    - It loads the tallies into its in-memory mirror when it restarts.
    - `reset` writes the empty crowd view of every quiz, so the view exists before anyone submits.
  - **Query.** `quiz.crowd` returns `CrowdView`, or `404 unknown-quiz`.
  - **Rebuild trigger.** The read-model stamp is now `PROJECTOR_REVISION:fingerprint`. Revision 2 rebuilds any store
    an earlier projector built, so existing dev data gets its crowd folded from the log. The boot log line says
    "another catalog or projector revision".
- **README.** Updated sections:
  - the wire section (`quiz.crowd`);
  - Presence §15–§17: the room table with badges, quiz and thinking rooms, the watch protocol, and admission;
  - the boot step on the projector revision;
  - layout (`👪️crowd`, five queries).

### Tests and gates (all run)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-server -p teaching-proctor` | server lib 99 (+4 watch), closed_ports 5, wire 4; proctor lib 44 (presence 6, crowd 2, projections +1, queries/instance extended), conformance 14, end_to_end 5 (+1) |
| `cargo clippy -p semio-framework-server -p teaching-proctor --all-targets --features semio-framework-server/conformance --no-deps -- -D warnings` | clean |
| `RUSTDOCFLAGS=-D warnings cargo doc -p semio-framework-server -p teaching-proctor --no-deps` | clean |
| Server TS package `bun ./📜️script.ts test`, and `tsc --strict` on `🟦️.ts` | 15 passed, 0 type errors |
| `contract --owner "🎓️teaching/🛂️proctor"` | 0 breaches under `🎓️teaching` or `🖥️server`, in the listing and in the full `testing.json` |
| `verify taxonomy report --scope 🎓️teaching/🛂️proctor` | `clean=true`, after adding `👪️crowd` to `members-of-modules` in `🔣️taxonomy.json` |
| end_to_end and the watch unit test, 4 repeated runs | 4 of 4 passed |

- **Oracle (third party: the core).** `a_tally_folded_one_result_at_a_time_is_the_cores_crowd_view_bit_for_bit` runs
  300 random results across both fixture quizzes. They include:
  - dropped and repeated tasks and tasks of the wrong kind;
  - repeated items;
  - unknown ids;
  - values up to `1e+21` and `1.5e-7`.

  Every tally goes through a JSON store round-trip before each fold. After every fold, both tallies must equal
  `quiz::crowd_view(quiz, all results)` as `CrowdView` and as serialized bytes.
- **End to end: `learners_see_what_the_others_think_live_and_what_they_answered`.** Two thinkers and a home watcher
  run through the real gateway with `tungstenite`. The test checks:
  - one snapshot per watched scope, in scope order (leaderboard, quiz, thinking);
  - a draft is seen both by the joined peer and by the watcher, then a revised draft;
  - `value-unknown` is refused;
  - a drag of `kettle` in the quiz room is seen through the watch, and a drag of `pellets` is `id-unknown`;
  - a watch of `other-catalog/home` gets `forbidden other-catalog/home`;
  - the crowd shows `runs 0` before any submission and `404` for an unknown quiz;
  - after one perfect run and one run with sorting reversed, `quiz.crowd` is **byte for byte** equal to the core's
    `crowd_view` over the two run results, and the sorting means are 0.5;
  - the crowd is identical after a restart.
- **Runtime across languages.** `proctor_presence_smoke.ts` now also drives watching through the TypeScript twin
  against the real `proctor serve` binary. It logged:
  - an empty `watched` snapshot of `proctor-fixture/quiz/power/thinking`;
  - the thinker's matching draft as a non-snapshot `watched` frame;
  - the thinker in `left` after close;
  - `[DEBUG] presence smoke passed`.

### Files

- Framework:
  - `🧰️framework/🛍️products/🖥️server/🔨️modules/{🧬️contract,📡️gateway}/🦀️.rs`
  - `…/🔨️modules/📡️gateway/🧪️tests/🔬️unit/🦀️.rs`
  - `…/🧫️fixtures/🔌️wire/🔣️.json`
  - `…/🧪️tests/🔬️wire/{🦀️.rs,🟦️.ts}`
  - `…/🟦️.ts`
- Proctor:
  - `🎓️teaching/🛂️proctor/🔨️modules/👪️crowd/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}` (new)
  - `…/🔨️modules/{👥️presence,🔭️projections,❓️queries,🧩️instance}/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`
  - `…/🔨️modules/⌨️cli/🦀️.rs`
  - `…/🧪️tests/🌐️end-to-end/🦀️.rs`
  - `…/📦️packages/🦀️rust/🦀️.rs`
  - `…/README.md`
- Shared: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, one member name.
- Ticket: `proctor_presence_smoke.ts`.

### Hand-offs

- **Client (react agent).**
  - The home page can join `<catalog>/home` and send
    `encodePresenceFrame({type:"watch", scopes:[…], intervalMs:250})` for its page, quiz and thinking rooms, up to
    16 in all.
  - On a `watched` frame with `snapshot`, replace what it holds for that scope; otherwise merge the entries and drop
    the sessions in `left`.
  - Thinking drafts must use `values` for matching.
  - `quiz.crowd` is ready.
- **Core agents.** The proctor relies on the current `ThinkingAnswer`/`ThinkingMatchingAnswer`, `thinking_problem`,
  `thinking_scope`, `crowd_view` and `json_number_text`. Any change to `crowd_view`'s counting rules must be mirrored
  in `CrowdTally::fold`, and the oracle test catches a drift.
- **Earlier items still open.** Server-product taxonomy kinds (§10); the hub is not compiled on this host (§10).
