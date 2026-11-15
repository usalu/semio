# Proctor hardening before the redeploy

Scope: `🎓️teaching/🛂️proctor`. Ticket: ARCHITECTURE-COURSE-REDEPLOY-READINESS.

## Findings

### 1. `open()` applies the schema before refusing: CONFIRMED, fixed

Evidence (before): `🔨️modules/🗄️storage/🦀️.rs`, `Database::open` ran `pragma journal_mode=WAL`,
`synchronous`, `foreign_keys`, then `execute_batch(SCHEMA)` (all `CREATE TABLE IF NOT EXISTS`), and only then
`stamp_format` read the format row. A foreign SQLite file got the proctor tables, a v2 file got its header switched to
WAL and missing tables created, and a garbage file was touched by the pragmas, all before the refusal.
`memory()` had the same order.

Fix: `Database::open` now calls `inspect_before_opening` (a read-only connection, no write possible): absent or empty
file is accepted, a file whose `proctor_format` row matches is accepted, everything else (another version, foreign
tables, a format table without row or of another shape, a file that is no database) is refused with the file
untouched. Only then are pragmas set and `set_up` runs one `IMMEDIATE` transaction that re-reads the identity, then for
an empty store creates the schema and the format row together (so a store is whole or absent), for a proctor store just
verifies, and rolls back on anything else (race with another process). An accepted store is no longer written at open.

Test: `a_refused_database_is_left_byte_identical_and_only_an_empty_file_is_set_up` (storage unit tests) covers an
earlier format in WAL mode, a foreign database, a format table without row, a format table of another shape and a
4 KiB garbage file: the main file is byte-identical afterwards, no WAL content, no schema created; an empty file is set
up and reopened.

### 2. Unthrottled CORS preflights: CONFIRMED, fixed

Evidence (before): `🔨️modules/🧩️instance/🦀️.rs`, `Proctor::assemble` built `server.throttled(served).layer(gatekeeping)`
and `gatekeeping` returned `preflight(...)` for every `OPTIONS` before calling `next`, i.e. outside the throttle. The
framework already classes `OPTIONS` as a read (`🖥️server/…/🛂️edge/🦀️.rs`, `request_class`), but the request never got
there.

Fix: the gate keeps transport trust and grants CORS on every response (including the throttle's `429`/`503`); the
`OPTIONS` answer moved into a new `preflighting` layer inside the throttle and outside the settling. A preflight now
spends its address's read allowance (default 600/s, burst 1800), a refused one is `429` + `Retry-After` + the CORS
grant (a page can read the wait) and no `Access-Control-Max-Age`; an admitted one is `204` with
`Access-Control-Max-Age: 7200` (Chromium's ceiling, so a session asks once per route and origin) and still never
reaches `consistency`/the routes, so it stays cheap. Docs: module header, `PREFLIGHT_MAX_AGE`, README "Cross-origin API".

Test: `a_preflight_spends_its_addresses_read_allowance_and_a_refused_one_is_granted_and_told_to_wait` (e2e edge):
flood from one address ends in `429` carrying allow-origin, expose-headers, `retry-after: 1`, no max-age; a foreign
origin gets the `429` without grant; another address still gets a cacheable `204`; reads of that address continue.
Existing learner-journey preflight test (granted/foreign, `max-age: 7200`) stays green.

### 3. Stale entry in `🎓️teaching/Cargo.lock`: REFUTED, nothing changed

- Working-tree lock: 276 packages, a reachability walk from the three workspace members
  (`teaching-proctor`, `-base`, `-domain`) reaches all 276; zero unreachable (script
  `scratchpad/proctor-lock-closure.py`, scratchpad only).
- Committed `HEAD` lock: 241 packages, 241 reachable; zero unreachable.
- `repo cargo --store=shared --configuration=prototype -- metadata --format-version 1 --locked --offline` exits 0 and
  leaves `Cargo.lock` byte-identical: cargo itself accepts the lock as complete and minimal (a stale entry would make
  `--locked` fail).
- Note for the committer: the working-tree `Cargo.lock` is already modified against `HEAD` (+438/-7 lines, the framework
  crates the proctor closure grew since the last commit; file mtime 05:03 today, not from this task). It must be
  committed together with the framework changes. `repo dependencies lock` regenerates `bun.lock`/package-manager locks
  of dependency owners, not this Cargo lock, so it was not run.

## Request-path review

Checked: `gatekeeping`/`consistency`/`preflight`/`grant`, the throttle + body taking (`body_taken`, `413`/`408`
already end-to-end tested), `ServerError` rendering, the deciders' and admission's input handling, config, health
probe, Caddyfile.

- Panics on malformed input: no `unwrap`/`expect`/`panic!` in the instance, queries, actors, projections, presence,
  crowd, course or storage non-test code; ids/handles are validated before use (README, already tested). OK.
- Unbounded body reads: the framework throttle takes bodies capped at `PROCTOR_LIMIT_BODY_BYTES` (default 16 KiB,
  1 MiB maximum) with a patience timeout (`408`), `413` beyond; Caddy also caps. OK.
- Error leakage: `ServerError::message` renders `Internal` as `"internal error"` and `StorageError::Backend` only reaches
  the log. The only reflected text is the JSON `404` naming method and path of the request itself. OK.
- Timeouts: the proctor itself sets no header-read or idle timeout (`axum::serve`); the production Caddyfile does
  (`read_header 10s`, `read_body 30s`, `idle 2m`, upstream dial 3 s, response header 30 s) and the proctor port is
  reachable by Caddy only. NOT changed: a header timeout in the proctor needs a replacement of `axum::serve` in the
  framework gateway (out of this scope); residual risk only when the port is exposed without the proxy, which
  the README forbids and `PROCTOR_TRUSTED_FORWARDING=proxy` already makes useless (cleartext requests get `403`).
- CORS for `https://architektur-und-technologie.de`: `PROCTOR_ALLOWED_ORIGINS` is parsed as exact serialized origins
  (no wildcard, path, userinfo; max 32), compared ASCII-case-insensitively, echoed, never `*`, no credentials.
  Added test `the_production_site_origin_is_granted_exactly_and_no_lookalike_is` (config): exact origin admitted;
  `http://` variant, `www.`, suffix/prefix look-alikes, other port, trailing slash, `null`, `*`, empty and a loopback
  origin are not; wildcard, trailing-slash and scheme-less allowlist entries stop the boot.
- Known, accepted: the `403 insecure-transport` and Caddy's own `413` carry no CORS headers, so a page would see an
  opaque failure instead of the status; both are not reachable on the intended deployment (Caddy always sets
  `X-Forwarded-Proto: https`; the client never sends a body above the limit).

## Files changed

- `🎓️teaching/🛂️proctor/🔨️modules/🗄️storage/🦀️.rs` (open, set_up, inspect_before_opening; stamp_format removed)
- `🎓️teaching/🛂️proctor/🔨️modules/🗄️storage/🧪️tests/🔬️unit/🦀️.rs` (new byte-identical test)
- `🎓️teaching/🛂️proctor/🔨️modules/🧩️instance/🦀️.rs` (preflighting layer, docs)
- `🎓️teaching/🛂️proctor/🧪️tests/🌐️end-to-end/🛡️edge/🦀️.rs` (preflight throttle test)
- `🎓️teaching/🛂️proctor/🔨️modules/🎚️config/🧪️tests/🔬️unit/🦀️.rs` (production-origin test)
- `🎓️teaching/🛂️proctor/README.md` (preflight admission, open-time inspection)

No new files or crates in the proctor, so the declared inputs and `proctorCargoManifests` needed no regeneration.

## Commands

All from `/Users/Shared/semio` with `SEMIO_LANGUAGE=en`, `--skip-remote-cache`, one at a time.

| Command | Exit | Result |
|---|---|---|
| `repo run @teaching/proctor:test` | 0 | nextest: 133 tests across 6 binaries, 133 passed, 0 skipped |
| `repo cargo --store=shared --configuration=prototype -- metadata --format-version 1 --locked --offline` | 0 | lock accepted, unchanged |
| `repo run @teaching/architecture-course:check` | 0 | green (declared proctor inputs cover every source) |
| `repo run @teaching/proctor:dev` (background, waited ~25 min in the capacity queue behind other agents) | stopped with SIGINT | logged `proctor listening on http://127.0.0.1:8791`, projections caught up at event 42 |
| `curl -sf http://127.0.0.1:8791/instance` | 0 | JSON of instance `teaching-proctor` v0.1.0 |
| `curl -i -X OPTIONS -H 'Origin: http://localhost:6061' -H 'Access-Control-Request-Method: POST' http://127.0.0.1:8791/commands` | 0 | `204`, allow-origin echoed, `access-control-max-age: 7200`, `vary: origin` |

After stopping, `lsof -iTCP:8791 -sTCP:LISTEN` lists nothing and no `proctor:dev` process remains.

Observation: the preflight now also carries axum's `allow: POST` header, because the layer sits on the method router and axum
appends `Allow` to a response its router did not match. Harmless to browsers; left as is.

## Unverified

- Per-test names are not printed by the repo test wrapper; that the three new tests ran is inferred from the compile
  of `teaching-proctor*` after the edits and the 133 total, not from a PASS line.
- The proctor's own lack of a header-read timeout is documented, not fixed.
