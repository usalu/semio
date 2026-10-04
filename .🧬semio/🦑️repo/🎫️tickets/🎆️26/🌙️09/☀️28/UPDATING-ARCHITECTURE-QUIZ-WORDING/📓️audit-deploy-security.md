# Audit: public deployment of the architecture quiz (operations and security)

Scope: site on GitHub Pages (`quizzes.architektur-und-technologie.de`), proctor API as Docker stack (proctor + Caddy) at
`proctor.quizzes.architektur-und-technologie.de`. Static reading only: nothing was built, run or exercised. Exploit
statements below are derived from the code paths cited and are marked "read, not run". Paths are relative to the repo root;
`DEPLOY` = `🎓️teaching/🏛️architecture/❓️quiz/🚀️deploy`, `GW` = `🧰️framework/🛍️products/🖥️server/🔨️modules/📡️gateway/🦀️.rs`,
`BUS` = `🧰️framework/🛍️products/🖥️server/🔨️modules/🎭️authority/🦀️.rs`, `INST` = `🎓️teaching/🛂️proctor/🔨️modules/🧩️instance/🦀️.rs`.

Count: 3 BLOCKER, 13 SHOULD, 12 NOTE.

---

## BLOCKER

### B1. Every learner id (the only credential) can be read by anyone, through the enrollment idempotency receipt
- Where: `BUS:294-302` (dedupe runs before policy, placement and decider, and returns the stored receipt as `Accepted`);
  `🎓️teaching/🛂️proctor/🔨️modules/🗄️storage/🦀️.rs:253-264` (`read_receipt` returns the receipt's `actor` tenant/kind/id, global
  key space, no principal or target check); `🎓️teaching/🛂️proctor/🔨️modules/🎭️actors/🦀️.rs:233` (the saga key is the public, guessable
  `enroll:<catalog>:roster:<seq>`, the receipt's actor is `quiz-learner/<learner id>`); `GW:1485-1501` (`post_command` returns the outcome verbatim).
- What happens (read, not run): `POST /commands` with an envelope whose `commandId` and `idempotencyKey` are
  `enroll:architecture:roster:1`, `target` `quiz-roster/roster`, any kind/payload (the shape is the one in `🎓️teaching/🛂️proctor/🧪️tests/🌐️end-to-end/🦀️.rs:106-127`)
  answers `{"status":"accepted","receipt":{"actor":{"kind":"quiz-learner","id":"<learner id>"},…}}`. Iterating `seq = 1…` returns the id of
  every learner ever registered or recalled, anonymous ones included (they never appear on the leaderboard). Past the head the same
  request falls through to the decider and is rejected `command-malformed`, which tells the attacker where to stop. With an id anyone can read
  `GET /actors/architecture/quiz-learner/<id>/events`, `quiz.learner`, start runs, record answers and submit on that learner. This voids the
  design statement that the leaderboard "never carries the learner id" (`📓️design.md` §13, `🧰️framework/🛍️products/❓️quiz/🧬️schema/🦀️.rs:728-740`).
- Smallest correct fix: in `BUS:294-302` namespace the idempotency key by principal (`principal_key|key` for lookup and for `record_receipt`), and treat a
  receipt as a duplicate only if `receipt.actor == envelope.target`; anything else is a rejection that carries no receipt data. Add an e2e test:
  register a learner, then `POST /commands` with `commandId = enroll:architecture:roster:1` as anonymous and assert the answer contains no learner id.

### B2. Client-chosen ids are never validated: enrollment poisoning, junk persisted at up to 2 MiB per id, unbounded in-memory activations
- Where: `🎓️teaching/🛂️proctor/🔨️modules/🎭️actors/🦀️.rs:162-180` (`admitted` checks equality of ids, never their shape); `🧰️framework/🛍️products/❓️quiz/🔨️modules/🧾️lifecycle/🦀️.rs:51-72,164-181`
  (no `Id`/`Slug` check; `Id` is only documented as "32 lowercase hex", `…/🧬️schema/🦀️.rs:22`); `BUS:310-322` (`activate` places and keeps an actor for
  any `target.id` before the decider can reject the command); `GW:2307-2326` + axum 0.8.9 default `Json` limit of 2 MiB.
- What happens (read, not run):
  1. Poisoning: an anonymous `identify-learner` with `id = enroll:architecture:roster:<future seq>` is accepted and its receipt is stored under that key. When the
     real learner with that roster seq registers, the saga's `quiz.enroll-learner` hits the dedupe branch (`BUS:294-302`), answers `Accepted` with no events, and the
     outbox row is marked delivered. The learner never gets `learner-registered` in its own stream, so every later `start-run` is `unknown-learner`, permanently,
     for every sign-up the attacker pre-poisoned (cost: one request each; reconcile re-submits the same keys and cannot heal it).
  2. Disk fill: `id`, `learner`, `run` accept strings up to the body limit. One accepted anonymous `identify-learner` with a 1 MiB `learner` is stored in the roster event,
     the outbox (payload and event_payload), the relayed learner event and the projection key. With no rate limit (B3) disk is exhausted by a few thousand requests.
  3. Memory: every rejected command for a fresh `learner` id still leaves an `Activation` in `AuthorityDirectory`, with the id as key, never passivated (`BUS:316-318` only on rehydrate error).
- Smallest correct fix: (a) B1's principal-namespaced keys remove the collision. (b) Validate shapes before placement: `Id` = `^[0-9a-f]{32}$` for `id`, `learner`, `run`, `Slug` via `is_slug`
  for `quiz`/`task`, applied to `envelope.target.id` too (add a pre-placement `admit` step to the `Decider` port, or reject in a proctor layer over `POST /commands`).
  (c) `DefaultBodyLimit::max(8 * 1024)` on the proctor router, and in `Caddyfile` `request_body { max_size 8KB }` as a second line. (d) Passivate an activation whose command was rejected at revision 0.

### B3. No flood protection of any kind on a public, password-less API
- Where: `INST:436-450` (the gate does protocol and CORS only); no rate limit, connection cap or concurrency cap exists in the proctor or the framework gateway (grep over
  `🎓️teaching`, `🧰️framework/🛍️products/🖥️server`: none; the proctor never reads `X-Forwarded-For` or the peer address); `DEPLOY/Caddyfile:8-16` (stock `caddy:2` has no `rate_limit` module and
  sets no `request_body`, timeouts or limits); `GW:1862` and `GW:901-912` (`ws.protocols(...)` leaves axum/tungstenite defaults: `max_message_size` 64 MiB, `max_frame_size` 16 MiB, verified in
  tungstenite 0.29 `protocol/mod.rs:101-102`; the 2 KiB state limit is checked only after the whole message is buffered); `🎓️teaching/🛂️proctor/🔨️modules/🗄️storage/🦀️.rs:45,110-123`
  (one SQLite connection behind a `std::sync::Mutex`, `synchronous=FULL`, blocking fsync on tokio workers); `INST:423-434` (`settle` = drain sagas + projection catch-up under the global
  `authority` and `projections` locks runs before every `POST /queries` and after every `POST /commands`); `GW:1240-1243` (`replay_events` holds the global command-bus mutex while reading a whole stream).
- What happens (read, not run): one unauthenticated client can (a) create unlimited learners, runs and events at wire speed, (b) serialize the whole API behind the single bus mutex and
  one fsync per command, (c) hold unlimited presence sockets (each 256 KiB of buffers plus a broadcast lane; a room roster is sent in full to every joiner and every batch goes to every member,
  so presence is O(N²) bandwidth) and send one 64 MiB text frame per socket, (d) open `/actors/<tenant>/<kind>/<id>/events/ws` for arbitrary ids (a lane per id).
  Legitimate use (a class of a few hundred) is unaffected; one script is enough to take the service down.
- Smallest correct fix that keeps the stack zero-touch (Caddy cannot do it without a custom build): in `INST:436-450` add a per-client-IP token bucket keyed by the single value Caddy writes into
  `X-Forwarded-For` (trusted only when `Forwarding::TerminatingProxy`), answering `429` with `Retry-After`; suggested buckets: `POST /commands` 20/s burst 60, `POST /queries` 10/s, WebSocket upgrades
  20/min with at most 6 concurrent per IP and a global socket cap, plus a prune of idle buckets. Add `ws.max_message_size(2048 + 64).max_frame_size(2048 + 64)` at `GW:1862`. Add `mem_limit` and `pids_limit`
  to the proctor service (S6). Caveat to verify with `curl -6` before relying on per-IP limits: Docker's userland proxy can replace the client address for IPv6, so XFF would be the bridge gateway for all v6 clients (N10).

---

## SHOULD

### S1. Handle recall is account takeover by design; the handle policy is client-side only
- Where: `🧰️framework/🛍️products/❓️quiz/🔨️modules/🧾️lifecycle/🦀️.rs:28-31,51-72`; `📓️design.md` §8, §13.
- Behaviour: a known handle returns `learner-recalled` with the claimed learner id to anyone (design, intended). Every handle is on the public leaderboard, so every named learner can be taken over in two requests: read the
  history, start or void runs, answer and submit an open run with arbitrary answers (score tampering through impersonation, no server bug involved). `Name` handles are real names.
  Each recall also appends a `learner-recalled` event to the victim's stream (unbounded, and it bumps `lastActivity`: `…/🧾️lifecycle/🦀️.rs:232`).
  The server normalizes only whitespace and `to_lowercase` (the NFC the design relies on is done by the client, `📓️design.md` §8): `U+200B`, bidi overrides (`U+202E`), zero-width and control characters pass,
  a handle can be a single invisible character, and homoglyphs (Cyrillic `а`) or NFD/NFC variants register as distinct, visually identical handles.
- Fix: owner decision first; state the threat model in the operator README and the identity screen. Cheap server-side hardening in `normalize_handle` using `std` only: reject `char::is_control`, the Unicode format
  characters (`U+00AD`, `U+200B-200F`, `U+202A-202E`, `U+2060-2064`, `U+2066-2069`, `U+FEFF`), and require non-whitespace characters to be `char::is_alphanumeric` or in ``-_.'`` (excludes Zalgo and invisible handles, requires NFC input as designed); do not append a `learner-recalled` event to a learner stream per recall (relay only the first).

### S2. Attacker-controlled state grows with O(N) work per command and O(N²) work per restart
- Where: `🎓️teaching/🛂️proctor/🔨️modules/🎭️actors/🦀️.rs:94-106` (the roster decider deserializes the whole `RosterState` handle map for every command and re-serializes it on every evolve); `BUS:310-322,386-394` (no snapshots are ever written, so
  `rehydrate` replays the whole stream through that O(N) evolve at the first command after each start, under the global bus lock); `🎓️teaching/🛂️proctor/🔨️modules/🔭️projections/🦀️.rs:94-99,198-201` (every learner state of every learner is mirrored in memory and the whole
  leaderboard is recomputed and rewritten as one blob on every settle that touched any learner); `🎓️teaching/🛂️proctor/🔨️modules/❓️queries/🦀️.rs:69-72` (`quiz.leaderboard` returns all rows, and each open home tab polls it every 10 s, `LEADERBOARD_POLL_MS`).
- Effect (read, not run): with N distinct pseudonyms the roster command costs grow linearly and the first `identify-learner` after a restart replays N events each costing O(N), while every query waits behind that lock (`INST:423-434`); a spam-filled leaderboard is shipped to every browser every 10 s. Unbounded runs per learner (`start-run` after each submit) grow the same way.
- Fix: write roster/learner snapshots in the bus (the `snapshot` store methods exist and `rehydrate` reads them, `BUS:386-394`); hold the handle index in a projection or SQL table instead of the actor state; cap total registrations and runs per learner (a rejection code in the core); serve only the top 100 rows plus the caller's own row; make the pre-query settle single-flight and skip it when the log head equals the checkpoint.

### S3. The release pipeline can push an untested image to the production tag from any branch
- Where: `.github/workflows/architecture-quiz.yml:65-87` (job `proctor`: no `environment`, no branch condition, no check before the push); `DEPLOY/🟦️.ts:207-219` (`docker-image-publish` only requires an existing image); `DEPLOY/Dockerfile:37-42` (the catalog is copied, never validated at build).
- Effect: any collaborator can dispatch the workflow from a feature branch and overwrite `ghcr.io/usalu/architecture-quiz-proctor:latest`, which `docker compose up` consumes. A release whose catalog is invalid or whose image does not boot is pushed anyway and then crash-loops on the host (`restart: unless-stopped`) after the old container was replaced. `docker-image-check` and `docker-stack-check` exist but run only by hand.
- Fix: `if: github.ref == 'refs/heads/main'` on both release jobs (or an `environment: production` with required reviewers and a branch rule); run `bun nx run @teaching/architecture-quiz:docker-image-check` between build and publish (add it to `dependsOn` of `docker-image-publish` in `📋️project.json`); add `RUN /out/release/proctor check /stage/content/🏛️architecture/❓️quiz/🔣️.json` after the stage step in the Dockerfile.

### S4. No rollback target: both tags move on every release
- Where: `Cargo.toml:296` (`version = "0.1.0"`, constant) and `DEPLOY/🟦️.ts:207-219` (tags `<workspace version>` and `latest`); `DEPLOY/compose.yaml:17` (`PROCTOR_TAG:-latest`).
- Effect: the second publish overwrites `:0.1.0` as well as `:latest`; the previous image survives on the host only as an untagged layer. README "Update" has no rollback step.
- Fix: also push `:sha-<GITHUB_SHA>` (immutable) and print the digest; document `PROCTOR_TAG=sha-… docker compose up -d` as the rollback and record the running digest before each update.

### S5. Workflow supply chain: tag-pinned actions, persisted credentials, lifecycle scripts next to a write token
- Where: `.github/workflows/architecture-quiz.yml:37-48,63,72-85` (`@v4`, `@v3`, `@v2` tags; `actions/checkout` keeps the job token in `.git/config`; `bun install` runs dependency lifecycle scripts in the `proctor` job whose token has `packages: write`).
- Fix: pin every action to a full commit SHA; `persist-credentials: false` on checkout; in the `proctor` job install with `bun install --frozen-lockfile --ignore-scripts` if the verbs allow it, otherwise move the `docker/login-action` step after the build and before the push only.

### S6. Compose has no log rotation, no resource limits, no container hardening
- Where: `DEPLOY/compose.yaml:16-52`.
- Effect: Docker's default `json-file` driver never rotates; both services log forever (Caddy logs TLS noise and ACME renewals, the proctor logs `[ERROR]` lines under attack). No memory or pid limit, so the unbounded growth in S2/B2 can take the host down, not just the container.
- Fix, per service: `logging: { driver: json-file, options: { max-size: "10m", max-file: "5" } }`; for `proctor`: `mem_limit: 1g`, `pids_limit: 512`, `read_only: true`, `cap_drop: [ALL]`, `security_opt: ["no-new-privileges:true"]` (it writes only `/srv/quiz/data`).

### S7. Backup is a manual stop-and-copy; no hot backup, no schedule, no copy before an update
- Where: `🎓️teaching/🏛️architecture/❓️quiz/README.md:95-116,118-128`; `DEPLOY/compose.yaml:5-6,29`; `🎓️teaching/🛂️proctor/README.md:189-201`.
- Effect: the only state is one SQLite file on a named volume; nothing runs a backup, nothing leaves the host, README "Update" does not back up first, and the image has no `sqlite3` or backup command, so a running proctor cannot be copied safely. Rollback across an event-schema change is unsafe (`evolve` silently ignores events it cannot decode, `🎓️teaching/🛂️proctor/🔨️modules/🎭️actors/🦀️.rs:102,129`), so the pre-update copy is the only way back. Backups hold names (S10) unencrypted.
- Fix: a `proctor backup <dir>` subcommand using SQLite's online backup API (`rusqlite` `backup` feature or `VACUUM INTO`) callable as `docker compose exec proctor proctor backup /srv/quiz/data/backup`; put "backup, then pull, then up" into README "Update"; schedule it on the host and copy off-host.

### S8. Operator docs drift from the files
- `DEPLOY/compose.yaml:5` says `docker compose up -d` is "first start and every update", but `pull_policy: missing` (`:18`) never refreshes `latest`; README "Update" (`🎓️teaching/🏛️architecture/❓️quiz/README.md:120-123`) correctly needs `docker compose pull proctor` first. An operator following the compose header runs stale code.
- `🎓️teaching/🛂️proctor/README.md:9-10,128` says the proctor serves the gateway routes and any other path is a JSON 404, but `GW:2307-2326` also mounts `/blobs/{hash}` (PUT/GET/HEAD, policy-denied), `/apps*` (empty) and `POST /scopes/{scope}/ephemeral` (policy-allowed inside catalog rooms, published to a lane nobody subscribes to). Each is attack surface that answers (a `Json`/`Bytes` body up to 2 MiB is read before the policy denies).
- Fix: correct both texts; mount only the five used routes in the proctor (a `base_router` variant without blobs, apps and ephemeral).

### S9. Custom-domain verification and subdomain-takeover coupling are not documented
- Where: README DNS block `🎓️teaching/🏛️architecture/❓️quiz/README.md:54-60`; `.github/workflows/architecture-quiz.yml:5-7`.
- Effect: the proctor grants CORS (and the API is meant to be used) from exactly `https://quizzes.architektur-und-technologie.de`. Without GitHub Pages domain verification, a repository rename or deletion lets someone else claim that host on their own Pages site and obtain the origin grant.
- Fix: add Settings → Pages → "Verify domain" with the `_github-pages-challenge-usalu.quizzes.architektur-und-technologie.de` TXT record to the README one-time steps. Also correct `.github/workflows/architecture-quiz.yml:3` ("the artifact's CNAME"): with the "GitHub Actions" Pages source the `CNAME` file is ignored, the custom domain is the repository setting.

### S10. No privacy notice, imprint or erasure path for personal data
- Evidence: no `Impressum`, `imprint`, `Datenschutz` or privacy text anywhere under `🧰️framework/🛍️products/❓️quiz/🎯️targets`, `🎓️teaching`, the README or `🌐️.html`. `Name` handles are published on the leaderboard, in presence and in the append-only event log; no command or tool deletes a learner (`proctor` has `serve|check|rebuild` only, `🎓️teaching/🛂️proctor/🔨️modules/⌨️cli/🦀️.rs:32-40`); backups (S7) keep them.
- Fix: before the public launch add an imprint and a privacy notice link to the site shell, say on the identity screen that names are public, and add an operator procedure for erasure on request (an `erase-learner` admin verb that rewrites the handle in a documented way, or a documented SQLite edit plus `proctor rebuild`).

### S11. Error bodies expose internals
- Where: `GW:136-175` (`ServerError::Internal(detail)` from `StorageError::Backend` renders raw SQLite text; `StorageError::Conflict` renders "idempotency key … is already bound to command …"); `BUS:411-415` (`unavailable` returns the storage error text in `reason` and `notices`); `🧰️framework/🛍️products/🖥️server/🔨️modules/🛡️policy/🦀️.rs:143-171` (deny reason names principal key, resource, grant and scope verbatim to the caller); `GET /instance` returns the full module manifest including every policy template.
- Fix: map `Backend` and `Internal` to a fixed `"internal error"` body and log the detail to stderr; return a fixed `forbidden` for policy denials on the public edge.

### S12. Caddyfile sets no body, time or log policy
- Where: `DEPLOY/Caddyfile:8-16`.
- Effect: Caddy forwards request bodies of any size to the proctor (the proctor's own 2 MiB default is the only cap), applies no read-header or read-body timeout, and writes no access log, so there is neither a slow-client defence nor any trace of who abused the API after the fact.
- Fix: inside the site block `request_body { max_size 8KB }`; in a global block `servers { timeouts { read_header 10s read_body 30s idle 2m } }`; and, only if the owner accepts keeping IP addresses (GDPR, retention), `log { output stdout format json }` with the S6 rotation.

### S13. Site security headers: nothing is shipped and Pages cannot set any
- Where: `DEPLOY/🟦️.ts:60-62` (`QUIZ_SITE_HEADERS` carries only `Cache-Control`); GitHub Pages ignores `_headers`; the host HTML contains inline `<script>` and `<style>` boot tags (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:448,734,747`).
- Effect: no CSP, no `frame-ancestors`, no `nosniff`, no `Referrer-Policy` on the site, whose origin holds every learner id in `localStorage` (`🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/💾️persistence/🟦️.ts`). I found no XSS sink in the quiz client (see Verified OK), so this is defence in depth, not a known hole.
- Fix: a build step that adds `<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'self' 'sha256-…'; style-src 'self' 'sha256-…'; font-src 'self'; img-src 'self' data:; connect-src https://proctor.quizzes.architektur-und-technologie.de wss://proctor.quizzes.architektur-und-technologie.de; base-uri 'none'; form-action 'none'">` with hashes of the inline blocks (feasible on Pages; `frame-ancestors` is not honoured in a meta tag, so clickjacking protection needs a CDN that honours `_headers`); extend `QUIZ_SITE_HEADERS` with `/*` rules (CSP, `X-Content-Type-Options`, `Referrer-Policy`, `frame-ancestors 'none'`) for such a CDN.

---

## NOTE

- N1. `INST:464-478` and `GW:1074-1082` send `Access-Control-Allow-Credentials: true`; the quiz client uses no cookies and no `Authorization`. Drop it from the proctor's `grant` (nothing breaks, one less widening).
- N2. CORS and the presence Origin check are hygiene, not abuse control: there are no ambient credentials, so any non-browser client ignores both. `admit_presence` checks the Origin only when one is present (`GW:1815`) and `/actors/…/events/ws` (`GW:1613-1625`) checks none; neither matters without cookies.
- N3. Quiz solutions are in the public repository (AGPL) and in the image; sheets are solution-free and the server scores (`…/🧾️lifecycle/🦀️.rs:207-225`), but `seed = run_seed(run)` of a client-chosen run id lets anyone grind for a favourable `draw` offline, and the crowd view shows the majority answers before a learner submits (intended, `📓️design.md` §17). The leaderboard is therefore for fun only; say so in the UI.
- N4. Pages answers deep links with `404.html` and HTTP 404, and caches `index.html` for 10 minutes; a visitor holding the old HTML after a deploy can request a hashed chunk that no longer exists. `_headers` is ignored there (documented).
- N5. Floating tags: `DEPLOY/Dockerfile:17-18` (`rust:1-bookworm`, `debian:bookworm-slim`), `DEPLOY/compose.yaml:37` (`caddy:2`). The runtime image installs `curl` and `ca-certificates` only for the `HEALTHCHECK` and the (absent) outbound calls; a `proctor health` subcommand would drop both.
- N6. No monitoring or alerting exists (no uptime probe, disk usage, certificate expiry). A reboot after `docker compose stop proctor` (backup) leaves it stopped (`unless-stopped`).
- N7. `docker-image-check` closes its sockets before `docker stop` (`DEPLOY/🟦️.ts:292-294,351-354`), checks only that `proctor.sqlite` exists (`:356-358`, not that the `-wal` was folded) and never sends a client-forged `X-Forwarded-Proto` through Caddy. Cheap additions.
- N8. During a restart Caddy answers 502 without retry (`reverse_proxy` default `lb_try_duration 0`); after a projector-revision bump the boot catch-up can exceed the 20 s start period plus 5 × 15 s retries, and `docker compose up -d` then reports an unhealthy dependency although the proctor becomes healthy.
- N9. `docker compose down --volumes` on the production host deletes `proctor-data` and `caddy-data`; the README never warns (the check verbs use a throw-away project name and cannot hit it).
- N10. IPv6: the README asks for an AAAA record; with Docker's userland proxy the client address of IPv6 connections can be masked. Test `curl -6` against a logged `X-Forwarded-For` before basing B3's limits on it, and fall back to limiting per /64.
- N11. Not verified on Linux: no run of `publish` (Vite/bun) or of the image build on Linux is recorded in the tickets; the workflow is the first. Run `bun nx run @teaching/architecture-quiz:docker-image-build` and `docker-image-check` on a Linux or devcontainer host before the first public push. A cold CI build has no BuildKit cache (the `--mount=type=cache` mounts at `DEPLOY/Dockerfile:33-34` do not persist on a hosted runner); expect a long first build.
- N12. Presence is unauthenticated: tag, identity and `surface` are client-asserted (`🧰️framework/🛍️products/❓️quiz/🔨️modules/👥️presence/🦀️.rs:50-61`), so a socket can show any handle or tag of another learner. Fine for "for fun"; do not build trust on it.

---

## Verified OK

- Production gate refuses to boot without mode, origin allowlist and `proxy` forwarding on a network bind; an empty `PROCTOR_ALLOWED_ORIGINS` in `.env` fails closed: `🎓️teaching/🛂️proctor/🔨️modules/🎚️config/🦀️.rs:112,146-153,223-226`.
- CORS: exact (case-insensitive) allowlist match, never `*`, `Vary: Origin`; the gate is the outermost layer and also wraps the fallback, and it removes the framework's reflective `ACAO`/`ACAC` before re-granting: `INST:329-333,464-478`, `🎓️teaching/🛂️proctor/🔨️modules/🎚️config/🦀️.rs:178-183`; `Access-Control-Allow-Headers: content-type` matches what the client sends (`🧰️framework/🛍️products/🖥️server/🟦️.ts:1131-1139`).
- Presence: Origin refused when present and not allowed, surface ≤ 64 chars, policy limits joins and watches to the catalog's own rooms, 2 KiB state and 30 frames/s per socket, ≤ 16 watched scopes, interval clamped, forwarders aborted on drop: `GW:901-918,1814-1827,1873-1938`, `INST:322-327`.
- No learner id in the leaderboard (tag only), crowd, presence or presence session ids (random, server-generated): `…/🧬️schema/🦀️.rs:728-740`, `GW:993-1008`. (The only leak is B1 and the by-design recall in S1.)
- Sheets are solution-free (`…/🃏️sheet/🦀️.rs`, `…/🧬️schema/🦀️.rs:286-370`); the server scores whole runs from stored answers; event time is the server's (`…/🧾️lifecycle/🦀️.rs:207-225`, `BUS:346,419-421`); the envelope principal is overwritten server-side (`GW:1485-1489`).
- SQL: every statement parameterized, DB path fixed (`🎓️teaching/🛂️proctor/🔨️modules/🗄️storage/🦀️.rs:72-74` and the grep over all `execute`/`prepare` calls); catalog and data paths come from the environment only; the traversal-guarded `StaticAppHost` serves nothing (no app registered).
- serde_json recursion limit applies (no stack overflow on nested JSON); all answers are structurally validated against the sheet before they are stored (`…/✅️validation/🦀️.rs:176-189`).
- No XSS sink (`dangerouslySetInnerHTML`, `innerHTML`, `eval`) in the quiz client, `ui-react` or the TS server client; routes are hash anchors, so no learner or run id appears in a URL.
- Image: non-root `quiz`, data dir `0700` and chowned before `VOLUME` (named volume inherits it), no secret or repository in the final image, tini as PID 1, `HEALTHCHECK` shell form expands `${PROCTOR_PORT}`, content read-only: `DEPLOY/Dockerfile:51-88`.
- Graceful stop is not blocked by open presence sockets: hyper completes the connection future at the WebSocket upgrade, so axum's graceful shutdown does not wait for them (axum 0.8.9 `serve/mod.rs` `handle_connection`, hyper 1.10.1 `server/conn/http1.rs:554-562`); SIGTERM through tini reaches `termination()` (`🎓️teaching/🛂️proctor/🔨️modules/⌨️cli/🦀️.rs:177-185`).
- Build context: emoji paths in the Dockerfile are byte-identical to the committed names (`grep -F` against `git ls-files`); no workspace member (277) and no path dependency (199) has a segment excluded by `.dockerignore`; no git dependencies; no build-time `include_*` of ignored files; `--locked`; `.gitattributes` forces LF (`i/lf w/lf` for every deploy file).
- Compose and Caddy: proctor port `expose`d only, Caddy waits for `service_healthy`, named volumes for data, certificates and config, `restart: unless-stopped`, 30 s stop grace; HSTS without `includeSubDomains`, `nosniff`, `no-referrer`, no second source of CORS headers; the volume name in README backup steps matches `name: architecture-quiz` plus `proctor-data` (`DEPLOY/compose.yaml:13,54-57`); config values (hosts, image, port 8791) agree across `🔣️.json`, Dockerfile, compose, Caddyfile and are asserted by `🧪️tests/🧪️deploy`.
- Workflow: manual dispatch only, top-level `contents: read`, minimal per-job permissions (`pages: write` + `id-token: write` for Pages, `packages: write` for GHCR), no stored secrets, `GITHUB_TOKEN` passed via `with:` (masked), `concurrency` without cancel, runs the repo verbs (`bun nx run …`), site artifact path matches `QUIZ_PAGES_DIRECTORY`.
- CDN artifact: `index.html`, `404.html`, `.nojekyll`, `CNAME` equal to the site host, the baked proctor origin and no loopback address are enforced before staging (`DEPLOY/🟦️.ts:160-169,174-191`); the build bakes `VITE_PROCTOR_URL` only for `build`, sourcemaps off, console stripped (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:65-70`).
