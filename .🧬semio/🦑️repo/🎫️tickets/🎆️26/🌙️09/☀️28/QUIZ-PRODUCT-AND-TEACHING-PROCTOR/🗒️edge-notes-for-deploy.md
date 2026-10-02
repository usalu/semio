# 🗒️ Edge notes for the agent "deploy"

Written by the agent "edge". **Everything below is final** (2026-10-02, after the capacity gate passed).

## 1. Request body limit — final

`16384` bytes (`PROCTOR_LIMIT_BODY_BYTES`, default `16384`). Your Caddyfile's
`request_body { max_size {$PROCTOR_LIMIT_BODY_BYTES:16384} }` is exactly right — keep it. It is now more than a
convenience: the proctor reads every body it accepts and every body of a request it refuses (so that the proxy's
connection to it stays open), but a body that *declares* more than the limit is answered `413` unread and that
connection closes. If Caddy let such bodies through, a flood of them would churn Caddy's connections to the proctor.

Measured, not guessed (`edge_measure_command_size.ts` in the ticket folder, run against the architecture catalog): the
largest legitimate `POST /commands` body is 2952 bytes (`record-answer` of `cooling/cooling-load-and-demand` with every
item), an `identify-learner` with a 64-character four-byte handle is 2185 bytes, a query is under 500 bytes. 16 KiB is
5.5 times the largest command and 128 times below the old 2 MiB default.

A body must also **arrive within 10 seconds** (fixed, not configurable): after that the proctor answers `408`
`{"kind":"stalled",…}` and closes. Your `read_body 30s` is looser than that and can stay; Caddy streams the body, so
the proctor's 10 s is what a slow client meets.

## 2. `PROCTOR_*` variables of the edge — final

All optional. Compose needs to pass none of them: the defaults are the production sizing (a class of 300 behind one
address), confirmed by the capacity gate. List them in `.env` only if you want operators to see the knobs.

| Variable | Default | Range | Meaning |
|---|---|---|---|
| `PROCTOR_LIMIT_BODY_BYTES` | `16384` | 1024…1048576 | largest request body |
| `PROCTOR_LIMIT_COMMANDS_PER_SECOND` | `300` | 1…1000000 | per client address: sustained `POST /commands` |
| `PROCTOR_LIMIT_COMMANDS_BURST` | `900` | 1…1000000 | per client address: burst of `POST /commands` |
| `PROCTOR_LIMIT_QUERIES_PER_SECOND` | `600` | 1…1000000 | per client address: sustained reads (`POST /queries`, every `GET`) |
| `PROCTOR_LIMIT_QUERIES_BURST` | `1800` | 1…1000000 | per client address: burst of reads |
| `PROCTOR_LIMIT_UPGRADES_PER_SECOND` | `100` | 1…1000000 | per client address: sustained WebSocket upgrades |
| `PROCTOR_LIMIT_UPGRADES_BURST` | `1800` | 1…1000000 | per client address: burst of WebSocket upgrades |
| `PROCTOR_LIMIT_SOCKETS_PER_ADDRESS` | `2048` | 1…1000000 | open WebSockets one client address may hold |
| `PROCTOR_LIMIT_SOCKETS` | `8192` | 1…1000000 | open WebSockets in total |
| `PROCTOR_LIMIT_IN_FLIGHT` | `2048` | 1…1000000 | requests served at once in total |
| `PROCTOR_LIMIT_PRESENCE_FRAME_BYTES` | `4096` | 4096…65536 | **new**: the most one presence frame carries (per socket and tick; per socket and watch interval over everything it watches) |
| `PROCTOR_LIMIT_PRESENCE_BYTES_PER_SECOND` | `16777216` | 65536…4294967296 | **new**: the most all presence rooms together send per second |

The domain agent's two caps are separate: `PROCTOR_MAX_LEARNERS` (`10000`) and `PROCTOR_MAX_RUNS` (`1000`).

**Uplink.** Presence is the proctor's bandwidth: 300 learners moving their pointers were sent 11 MiB per second in the
gate (the budget above is 16 MiB per second = 134 Mbit/s). If the host's uplink is smaller, set
`PROCTOR_LIMIT_PRESENCE_BYTES_PER_SECOND` to what it can carry; presence then simply updates less often, nothing else
changes. Commands and queries are a few hundred kilobytes per second.

## 3. What the image check and the stack check must stay under

- **Client address.** With `PROCTOR_TRUSTED_FORWARDING=proxy` the proctor takes the client address from the **last**
  value of `X-Forwarded-For` (what the terminating proxy wrote; Caddy's `reverse_proxy` replaces the header with the
  remote address by default, which is exactly that). A request without the header — your check on the direct loopback
  port, the image `HEALTHCHECK` — is keyed by its peer address. IPv6 addresses are keyed per /64. Nothing else about
  `X-Forwarded-*` changed: `X-Forwarded-Proto: https` is still required, `X-Forwarded-Host` is not read.
- **Budget from one address:** 900 commands at once, then 300 per second; 1800 reads at once, then 600 per second; 1800
  socket upgrades at once, then 100 per second; 2048 sockets open at a time. Your ~20 requests in a few seconds and a
  handful of presence sockets are three orders of magnitude below every one of them.
- **What a refusal looks like** (so a check can assert it never sees one): `429` with `Retry-After: <seconds>` and
  `{"kind":"throttled","message":"too many requests","retryAfterMs":…}` for an address over its allowance; `503` with
  `Retry-After` and `{"kind":"overloaded",…}` at the total caps; `408` `{"kind":"stalled",…}` for a body that did not
  arrive within 10 s; `413` `{"kind":"payloadTooLarge",…}` past the body limit. All carry the CORS grant.
- **WebSocket frames:** a presence socket closes when one inbound message exceeds 4096 bytes (a state is at most 2048);
  the learner event socket (`/actors/…/events/ws`) closes on an inbound message over 1024 bytes. Your checks send
  neither. A presence socket that does not take a frame for 60 s is closed as well (a check that opens a socket and
  never reads it will see that).
- **Presence frames a check reads:** the `welcome` is still the first frame and is led by the joining session, but a
  roster larger than 4096 bytes continues in the `batch` frames that follow at once (same for a `watched` snapshot). A
  check that counts roster entries must apply the frames as a client does, not read the `welcome` alone. With a handful
  of sockets everything still fits the `welcome`.
- **Routes:** the proctor mounts only `GET /instance`, `POST /commands`, `POST /queries`,
  `GET /scopes/{scope}/presence/ws`, `GET /actors/{tenant}/{kind}/{id}/events` and `…/events/ws`. `/blobs/*`, `/apps*`
  and `POST /scopes/{scope}/ephemeral` answer the JSON `404`.
- **`GET /instance`** still answers `200` with `{"id":"teaching-proctor","version":…,"modules":[…]}`; each module's
  `policies` array is empty (the policy templates are no longer published). `proctor health` is unaffected.
- **CORS:** `Access-Control-Allow-Credentials` is no longer sent (the client sends no credentials);
  `Access-Control-Expose-Headers: Retry-After` is new on granted responses. If a check asserts the exact grant headers,
  it needs that change.

## 4. Things only the deployment can bound

- **Connections.** The proctor caps WebSockets and requests in flight, not TCP connections: an idle keep-alive
  connection costs it a few kilobytes and nothing bounds their number. Behind Caddy that is Caddy's pool (few
  connections); your `idle 2m` and `read_header 10s` cover the client side. Keep the proctor's port unreachable from
  anything but Caddy — the README's "nothing but the reverse proxy may reach the proctor's port" is load-bearing.
- **File descriptors.** 8192 sockets plus Caddy's upstream connections need a `nofile` limit above ~10000 for the
  proctor container (Docker's default is far above; a hardened host profile may not be).
- **Durability.** Facts (events, receipts, outbox rows, snapshots) are flushed to the disk at every commit as before
  (`synchronous=FULL`). Read models and outbox delivery marks now commit without waiting for the disk and ride along
  with the next flush of facts; after a power loss the proctor folds and relays again what was not on the disk. Nothing
  for the runbook to do — `backup`/`restore` and `health` are unaffected — but worth knowing when reading the WAL.

## 5. The capacity gate

`bun nx run @teaching/proctor:capacity` (launch row `⚖️gate🎓️teaching🛂️proctor🏋️capacity`) builds the release binary and
needs a quiet machine for about five minutes; it is not part of `test` and should not run in the same CI job as other
load. `-- --executable <path>` measures a binary you built elsewhere (the one of an image, extracted). It fails with
"this machine was too busy to measure the hall" rather than with latency numbers when the runner is overloaded.

## Status

- 2026-10-02 — sections 1 and 3 final; section 2 names final, numbers provisional until the capacity run.
- 2026-10-02 (later) — capacity gate passed; all numbers final; sections 2 (two presence variables), 3 (`408`, presence
  frames), 4 and 5 added.
