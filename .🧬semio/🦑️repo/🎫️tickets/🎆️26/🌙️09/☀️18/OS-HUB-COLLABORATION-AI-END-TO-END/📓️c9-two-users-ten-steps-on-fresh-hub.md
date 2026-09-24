# C9 — two users, ten steps

Slice C9 / ticket 26/09/18 / session-9 WP-O3 (2026-09-23 evening).

## 0. HANDOFF (read this first)

| field | value |
|---|---|
| infra | hub **7704** on `o3-boot-7704` (APFS clone of o3-boot); gis2d **6204**; peer **7681** left alone; **7702** contested by parallel agent — do not fight |
| binary | `.tmp-wp-o3/bin/os-hub` rebuilt with policy `authenticated` name-fallback + `ServerFrame::Session { actor: actor.to_string() }` |
| receipt `uiPatch.receipt: invalid bytes` | **CLEARED** (o3c9b+) |
| document WS **403** policy deny | **CLEARED** (o3c9e single-user sustained WS; dual raw WS holds 12s) |
| single-user mount (o3c9e) | **GREEN** — document/ws open never closed ~271s; stages → `actor-ready`; sync **Persisted** |
| collab o3c9i (stock steps, longer settle) | **6 PASS / 6 FAIL / 1 SKIP** — not 10/10 |
| honest green steps | 1a, 1b, 1c (soft — counts reconnect storm), 1e, 7 (vacuous zero writes), 8 |
| red | 1d/5 presence empty; 2/3/4/6 edits NO-OP |
| next blocker | browser closes document/ws in **~1–5 ms** with `1008 browser actor activation failed` / reconnect storm; UI shows "renderer is unavailable" + `Remote: backoff`; pageerror `document opening deadline exceeded` still seen once per user |
| dual raw transport | **OK** — `semio.session.v1` open+hold 12s sequential and parallel (`dual-session-ws.txt`) |
| zero-touch `dev s` | **not started** |
| gate | No |

## 1. Closed this session

1. Hub compile: `hub/documents` Session actor `String` (moved-value fix).
2. Hub rebuild via `fleet-mutex.sh hub wp-o3` → install to `.tmp-wp-o3/bin/os-hub`.
3. Isolated to **7704** after 7702 hold fights with parallel wp-o3 agent.
4. Policy name-fallback live in binary; single-user mount proves subscribe works.
5. Raised `BROWSER_ACTOR_CHILD_LIMITS` (boot/load/throughput) → ~498s `documentOpeningDeadlineMs` (deadline alone did not fix dual).

## 2. Dual-user failure mode (measured)

1. Playwright two contexts attach the same gis document.
2. Each document/ws **opens then closes in ~1–5 ms** (o3c9i `SOCKETS` lifetimes); dozens of cycles per user.
3. Store worker closes with **`1008 browser actor activation failed`** after `activateDocumentBrowserActorAfterSession` throws (`store-worker.ts`); status becomes integrity-failed / renderer-unavailable.
4. Presence roster stays empty; shell commands NO-OP (`instance-retired` / `view-state-unresolved` on earlier run).
5. Raw dual WebSocket without hello/activation **does not** flap — so gateway policy + upgrade are fine; failure is **post-open browser-actor activation / Session handshake** under dual load.

## 3. o3c9i step table

| step | result |
|---|---|
| 1a-boot | PASS |
| 1b-sign-in | PASS |
| 1c-both-attached | PASS (sockets=35/20 = reconnect storm, not sustained) |
| 1d-rosters | FAIL empty |
| 5-presence | FAIL |
| 1e-baseline | PASS (local inspector extents agree) |
| 2/3/4/6 edits | FAIL NO-OP |
| 7 convergence | PASS vacuous (0/10 writes) |
| 8 reload | PASS extents identical |
| 9 hub restart | SKIP (no C3_HUB_RESTART) |

## 4. Next actions (opinionated)

1. Capture the **integrity-failed diagnostic string** on dual attach (c4 probe already reads `data-semio-execution-target-diagnostic`) — distinguish `missing Session` / `session-mismatch` / `capacity` / activate load error.
2. Fix that cause (likely Session actor vs `pendingSocketActorId` / hello ClientFirst race under dual reconnect, not policy).
3. Re-run collab only after **one** sustained document/ws per user (open without close for ≥30s) + non-empty presence.
4. Then zero-touch `dev s` / launch.json / local-only when hub down.

## 5. Captures

`.tmp-wp-o3/generated/` — `o3c9e-*`, `o3c9f-*`, `o3c9i-*`, `dual-session-ws.txt`, `hub-7704.txt`, `serve-6204.txt`, `hub-rebuild-policy3.txt`, `add-member-7704.txt`
