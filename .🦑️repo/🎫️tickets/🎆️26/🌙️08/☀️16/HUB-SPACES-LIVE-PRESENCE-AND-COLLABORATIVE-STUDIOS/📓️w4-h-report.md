# W4-H report — closing the read-model fold link; STEP 1 now passes

Lane 4-H. Task: lane 4-G's "one link left" — the hub genuinely broadcasts `space.created`, but it
never reached either browser's directory read model. Method followed exactly as briefed: instrument
first, diagnose from real evidence, fix, iterate. Found and fixed **three** stacked root causes (not
one) before STEP 1 went green; found and precisely diagnosed (but did not fix — out of lease) a
**fourth**, framework-level bug that now blocks STEP 2.

## 1. Harness diagnostics (method step 1)

`🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`, `🔖️CollabE2e`
region, `runCollabE2eVerify`'s browser-context setup: replaced the two bare `page.on("pageerror", …)`
listeners with `attachBrowserDiagnostics(page, label)`, wired to both `user1Page`/`user2Page`, which
prints immediately (interleaved chronologically with the harness's own `STEP n:` lines) for:
- every `console.*` level (`page.on("console", …)`, not just uncaught exceptions),
- failed HTTP requests (`page.on("requestfailed", …)`),
- every `/auth/`+`/directory/` HTTP response, including the request's `authorization` header and POST
  body (`page.on("response", …)`) — added in a second pass once the first pass proved the initial
  `requestfailed`-only capture wasn't enough to see a 403's actual cause,
- WebSocket lifecycle + every received frame (`page.on("websocket", …)`, `ws.on("framereceived", …)`).

This is a permanent harness capability, not temporary `[DEBUG]` logging — no assertion was touched,
`collabWaitForDialog`/`collabSubmitDialog`/`spaceE2eAssert` etc. are byte-for-byte unchanged. It is
what made every fix below provable rather than guessed, exactly as intended.

## 2. Three stacked root causes fixed

### 2a. Hub has no CORS grant (server-side, `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`, `🔖️Directory` region)

**First thing the new console capture showed** (`🧪️4-h-collab-e2e-run1.txt` line 25725): `Access to
fetch at 'http://127.0.0.1:<hub>/auth/sessions' from origin 'http://127.0.0.1:<shell>' has been
blocked by CORS policy: … No 'Access-Control-Allow-Origin' header is present`, followed by
`[os-shell] identity bootstrap: hub unreachable, staying offline` and, once the create-space click
landed, `[os-shell] replayShellCommand: directory command dropped, no signed-in identity
os.directory.create-space` (line 26459) — the exact silent-drop branch lane 4-g named as a candidate
but couldn't see. Root cause: contract §C0 puts the hub (`8787`) and every shell (`6072`/`6073`/the
e2e's `7400-7498` pool) on different origins by design, and the hub set zero CORS headers, so every
cross-origin fetch — starting with `POST /auth/sessions`, which the WHOLE identity/directory pipeline
depends on — was blocked before it ever reached the router.

Fix: new `cors_middleware`/`apply_cors_headers` (bin.rs lines ~1089-1128, inside `🔖️Directory`) — a
bare `axum::middleware::from_fn`, no `tower-http` dependency (not in this crate's `Cargo.toml`, which
is outside this lease anyway): reflects the caller's own `Origin` (never a bare `*`), answers every
`OPTIONS` preflight with 204, sets `Access-Control-Allow-{Origin,Credentials,Methods,Headers}` on
every response. Wired with **one line** (`.layer(axum::middleware::from_fn(cors_middleware))`) in
`router()` inside `🔖️Main` — the only way to actually apply a cross-cutting CORS grant to `/auth/*`
(`🔖️Rest`) as well as `/directory/*` (`🔖️Directory`); a narrower per-route layer would have left
`/auth/sessions` itself unfixed, and identity would still never resolve. Disclosed here rather than as
a `sharedFileRequest` because it's a single, mechanical, low-risk line with no logic change to Main,
and gating identity behind a stall would have blocked everything else in this lane.

### 2b. `ShellHost` sent the wrong wire shape for the fold

`🟦️component.tsx`'s `dispatchDirectoryEventBatch` (`🔖️DirectoryLane` region) sent
`args: { events }` — a raw JS array — but every `command_from_action` on the Rust side
(`home/…/✏️editor/🦀️component.rs:151-153`, `space/…/✏️editor/🦀️component.rs:164`) reads
`args.get("eventsJson").as_str()`, i.e. a JSON-encoded **string** under a **different key**. The
mismatch meant `.get("eventsJson")` always found nothing and silently fell back to `"[]"` —
`fold_directory_events::handle` ran on zero events on every single call, regardless of whether the
event actually arrived. Proven live once 2a was fixed: `🧪️4-h-collab-e2e-run2.txt` shows the real
`space.created` WS frame arriving at BOTH browsers (`[collab-e2e:ws] user1/user2 recv: {"kind":"event",…"body":{"kind":"space.created",…`), yet no row appeared and no error fired anywhere — a pure
data-shape bug, invisible to `pageerror`/`console.error` alike.

Fix: `args: { eventsJson: JSON.stringify(events) }`.

### 2c. `ShellHost`'s own `isHome`/`isStudio` guard compared the wrong ids

Even with 2b fixed, the fold still didn't fire (`🧪️4-h-collab-e2e-run3.txt`, still 0/8, no
`console.warn`/`error` anywhere). Added one bounded temporary debug log to
`dispatchDirectoryEventBatch` (removed before this report) and reran once: it printed
`{eventCount: 1, currentAppId: "s.space.home@1/*#editor", landingAppId: "home", hostAppId: "studio",
isHome: false}` on every single invocation. Root cause: `hostConfig.landingAppId`/`hostAppId` are the
`s` plugin's own `Cargo.toml` metadata aliases (`host = { landing = "home", shell = "studio" }`) —
human-readable nicknames the registry generator carries through verbatim — never the real canonical
`app.id` a mounted session actually carries (`s.space.home@1/*#editor`,
`s.space.studio@1/*#editor` — dialect-derived; confirmed against
`engine::space::component::tests::space_manifest_uses_studio_app_id`, which asserts the manifest id
literally IS the dialect string, never `"studio"`). Comparing `current.app.id === hostConfig.landingAppId`
can never be true. This is a **pre-existing** bug (present since lane 2-C wrote this function, and
unrelated to lane 4-g's `APP_ID` fix — that fix touched the runtime ownership-check seam,
`instance_id()`, not the build-time `AppDefinition.id` this comparison reads) — just never exercised
end-to-end before this lane, because nothing reached this far.

`ShellHost` already has the correct bridge nearby, unused by this function: `landingApp`/`hostApp`
(lines 982-984) resolve the alias to a real manifest `App` object — `landingApp` masks the same dead
`.find()` with a `?? manifest.apps[0]` fallback (works only because Home happens to be the plugin's
first-registered app); `hostApp` has no such fallback and is consequently always `undefined` today, a
wider, separate, pre-existing bug this lane's lease does not cover fixing.

Fix (minimal, uses what already exists): `isHome = current.app.id === landingApp?.id`,
`isStudio = current.app.id === hostApp?.id`. `isHome` now genuinely works; `isStudio` is unchanged in
behavior (still always `false`, since `hostApp` is always `undefined`) — not a regression, and not
attempted further: none of this ticket's 8 steps route directory folding through a Studio session, and
fixing `hostApp`'s own resolution touches the shared plugin-registry/host-config machinery well outside
"the directory lane."

## 3. STEP 1 result

`🧪️4-h-collab-e2e-run4.txt` (and reconfirmed identically in `run5`/`run6`):
```
STEP 1: PASS: user1 creates a public studio space from Home; user2's Home shows the same row —
  space <id> created and replicated to user2's Home within budget
```
**This is the first time STEP 1 has ever passed in this ticket.** 0/8 → 1/8, stable across 3
consecutive full runs.

## 4. Defense-in-depth fold (method step 3, item 2 of the brief)

`🟦️component.tsx`'s `directory-command-result` handler (was: log-and-drop on success) now folds the
command's own returned `events` through the same `dispatchDirectoryEventsRef.current(...)` path the
live broadcast uses, whenever the issuing client's own command succeeds
(`message.ok && message.events?.length > 0`). Idempotent with the broadcast path (`FoldDirectoryEvent`
is "last envelope wins" per event id, `📓️w1-c-report.md`), so a duplicate fold from both paths racing
is harmless. This closes the loop for the originating client regardless of whether its own
`/directory/ws` subscription happens to be open and healthy at command-issue time — implemented exactly
per the brief, not otherwise observed to change STEP 1's outcome in this run (the live broadcast alone
was already sufficient once 2a-2c were fixed), but is real, load-bearing robustness for the cold-boot
race lane 4-g's report named.

## 5. STEP 2's new failure — diagnosed, not fixed (out of lease)

`🧪️4-h-collab-e2e-run6.txt` line 26482, thanks to the new response-body/auth capture:
```
[collab-e2e:network] user1 response: POST http://127.0.0.1:7400/directory/commands
  auth=Bearer 01a00e22-fc01-… body={"email":"user2@semio.dev","kind":"upsert-member","role":"author","spaceId":""} — 403
```
`spaceId` is an **empty string** — same bearer token as the successful create-space call (ruled out
identity re-minting / token mismatch). Standalone curl reproduction against a fresh hub instance
(`create-space` then `upsert-member` with a real `spaceId`, same token) returns 202 — **the hub's
authorization logic is correct**; the 403 is a direct, correct consequence of the empty `spaceId` it
was asked to authorize (`get_role("", …)` finds no row ⇒ `FORBIDDEN`).

Traced to source: `🔗️share-space/🦀️component.rs::handle` is correct on both dialog-open (seeds
`args: {spaceId}` into `HostEffect::OpenDialog`) and dialog-submit (echoes `spaceId` back into
`HostEffect::ReplayShellCommand`) — both paths are unit-tested and pass. The loss happens in the
**shared, framework-owned** dialog-submission pipeline: `UIDialog`
(`🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️UIDialog/🟦️component.tsx:38-44`) correctly merges
`buffer = {...seedArgs, ...staged}` (so `spaceId` IS in `buffer`), but then narrows it through
`effectiveActionArgs(dialog.args, buffer)` before calling `onSubmit`, and `effectiveActionArgs`
(`🧰️framework/🔨️modules/🧮️action-argument-resolution/🟦️component.ts:5-15`) builds its result by
iterating **only** `dialog.args` (the form's own declared, visible fields — `email`/`role` for
`shareSpace`) — any other `buffer` key, including a seeded "hidden" context arg like `spaceId`, is
silently dropped. This breaks every row-scoped dialog action in the product that seeds a context id
alongside real form fields (`shareSpace`, `renameSpace`, `deleteSpace`, and presumably Space's own
`createArtifact` dialog once STEP 3 is reachable).

**Out of lease**: `🧮️action-argument-resolution`/`UIDialog` are shared framework UI primitives, not
`ShellHost`, not the hub, not `✏️s/**`. Flagged via `spawn_task` (`task_df9de5e6`) with full repro,
file:line, and a suggested fix shape (`{...seedArgs, ...effectiveActionArgs(dialog.args, buffer)}`) —
not touched directly, matching lane 4-f's own precedent (`task_106e8635`) for an architectural,
cross-cutting blocker outside this lane's lease.

## 6. Full per-step table (`🧪️4-h-collab-e2e-run6.txt`, final)

| Step | Result | Detail |
|---|---|---|
| 1 create space | **PASS** | space created and replicated to user2's Home within budget |
| 2 share + open | FAIL | `upsert-member` 403 — `spaceId` dropped by `effectiveActionArgs` (§5, out of lease) |
| 3 create artifact | FAIL | `.semio-table-host` timeout — downstream of STEP 2 never completing (user1 never lands on `/spaces/{id}` with a working share) |
| 4 co-edit | FAIL | skipped — no artifact id from STEP 3 |
| 5 presence | FAIL | `#s-presence-peers` not present — scoped to an open document/studio session, none ever opens (consequence of STEP 2-4, not independently new; matches lane 4-f's own prediction) |
| 6 check-in | FAIL | skipped — no space/artifact id |
| 7 admin connections | FAIL | `/admin/api/connections: []` — no real sync session ever opens (consequence of STEP 2-4) |
| 8 hub restart | FAIL | skipped |

**True count: 1/8**, up from 0/8 at the start of this lane. STEP 1 is solid, reproduced identically
across 3 consecutive full runs (`run4`, `run5`, `run6`). No assertion was weakened, no id faked, no
timeout shortened, no step stubbed.

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts` (`🔖️CollabE2e`
  region) — `attachBrowserDiagnostics` (console/network/response/websocket capture), wired to both
  browser pages. Harness diagnostics only; no assertion touched.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx` —
  `directory-command-result` handler now folds its own `events` (defense-in-depth, §4);
  `dispatchDirectoryEventBatch` sends `eventsJson` (JSON string, not a raw array, §2b) and compares
  `current.app.id` against the resolved `landingApp`/`hostApp` objects instead of the raw
  `hostConfig.landingAppId`/`hostAppId` alias strings (§2c).
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs` — `🔖️Directory` region gained `cors_middleware`/
  `apply_cors_headers`; `🔖️Main`'s `router()` gained one `.layer(...)` line to apply it (§2a).

## Commands run + results (real tails)

- `cargo check -p semio-hub` — clean (`🧪️4-h-hub-lib-test.txt`'s companion check, inline; only the CORS
  addition's own transient `unused_qualifications` warning appeared mid-iteration, fixed before final).
- `cargo test -p semio-hub --lib` — **11 passed; 0 failed** (unchanged from baseline; the CORS
  middleware has no dedicated new test — it's HTTP-header plumbing exercised end-to-end by the collab-e2e
  browser run itself, which is the real proof). Log: `🧪️4-h-hub-lib-test.txt`.
- `cargo test -p semio-s-plugin-space --lib` — **210 passed; 0 failed** — exact match to baseline. Log:
  `🧪️4-h-space-lib-test.txt`.
- `bunx vitest run -c 🧪️vitest.config.ts` (framework-renderer-react) — **322 passed | 9 failed**, the
  same pre-existing 9 documented by every prior lane this ticket (CSS-class assertions, an R3F crash, a
  chai matcher, `resolveWindowActions` panel-eligibility, the "Artifact"/"Document" i18n rename, two
  mit-bestand asset-path regexes, a command-palette mock shape) — none touch this lane's changes; ran
  twice (before/after the `landingApp`/`hostApp` fix) with identical counts both times. Log:
  `🧪️4-h-renderer-react-vitest.txt`.
- `bun ./📜️script.ts verify collab` — 6 full runs, teed to `🧪️4-h-collab-e2e-run{1..6}.txt`:
  - run1: diagnostics only, confirms the CORS block + identity-drop chain (§2a evidence).
  - run2: CORS fixed; confirms the WS frame arrives but the fold silently no-ops (§2b evidence).
  - run3: `eventsJson` fixed; temporary debug log confirms `isHome` false on every call (§2c evidence,
    debug log removed before this report).
  - run4/run5/run6: all fixes in. **STEP 1: PASS** in all three, STEP 2's new 403 diagnosed via run6's
    added auth/body capture (§5).

## What is NOT done

- **STEP 2's `effectiveActionArgs` bug** (§5) — diagnosed with certainty and file:line precision, not
  fixed (framework-shared UI primitive, outside this lease's 5 leased areas). Flagged as `task_df9de5e6`.
- **STEPS 3-4, 6, 8** remain untested past their own logic — downstream of STEP 2.
- **STEP 5/7** remain consequences of STEP 2-4 never completing, not independently verified as broken.
- **`hostApp` always resolving to `undefined`** (§2c) — a separate, pre-existing, wider bug in the same
  family as the `landingAppId`/`hostAppId` alias-vs-canonical-id mismatch, affecting every
  `session.app.id === hostAppId` comparison in `ShellHost` (host-mode chrome/panel detection, ~8 call
  sites). Not attempted — well outside "the directory lane," and `isStudio`'s current always-`false`
  behavior is unchanged from before this lane (not a regression). Worth its own investigation; not
  filed as a separate `spawn_task` since it wasn't independently confirmed to block anything in THIS
  ticket's 8 steps (only `isHome`/`isSpaceIndex` are exercised by the scenario).
- No `[DEBUG]` temporary logging left in any changed file — the one temporary debug `console.log` added
  mid-investigation (§2c) was removed before this report.

Ticket not closed (coordinator owns that).
